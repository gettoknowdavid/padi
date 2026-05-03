use crate::errors::AppError;
use crate::features::auth::models::User;
use crate::features::auth::{RegisterRequest, UserResponse};
use anyhow::Result;
use deadpool_redis::Pool as RedisPool;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthRepository {
    pub database: Arc<PgPool>,
    pub redis: Arc<RedisPool>,
}

impl AuthRepository {
    async fn redis_conn(&self) -> Result<deadpool_redis::Connection, AppError> {
        self.redis.get().await.map_err(|e| {
            tracing::error!("Redis connection failed: {:?}", e);
            AppError::Internal("Redis connection failed".to_string())
        })
    }

    pub fn new(database: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
        Self { database, redis }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<UserResponse>> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"
            SELECT id, email, phone, full_name, is_verified
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.database)
        .await?;
        Ok(user)
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!""#,
            email
        )
        .fetch_one(&*self.database)
        .await
        .map_err(|e| {
            tracing::error!("DB error checking email: {:?}", e);
            AppError::Internal("Internal server error".to_string())
        })
    }

    pub async fn create_user(
        &self,
        req: &RegisterRequest,
        password_hash: String,
    ) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, full_name, phone, is_verified)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, phone, full_name, avatar_url, password_hash, is_verified,
                      failed_login_attempts, locked_until, created_at, updated_at, deleted_at
            "#,
            Uuid::new_v4(),
            &req.email,
            password_hash,
            &req.full_name,
            req.phone.as_deref(),
            false
        )
        .fetch_one(&*self.database)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            AppError::Internal("Failed to create user".to_string())
        })
    }

    pub async fn set_user_verified(&self, user_id: &Uuid) -> Result<(), AppError> {
        sqlx::query!("UPDATE users SET is_verified = true WHERE id = $1", user_id)
            .execute(&*self.database)
            .await
            .map_err(|error| {
                tracing::error!("Failed to verify user: {:?}", error);
                AppError::Internal("Failed to verify user".to_string())
            })?;
        Ok(())
    }

    pub async fn create_audit_log(
        &self,
        user_id: Uuid,
        action: &str,
        ip_address: Option<String>,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, ip_address)
            VALUES ($1, $2, $3, 'user', $4, $5)
            "#,
            Uuid::new_v4(),
            user_id,
            action,
            user_id,
            ip_address.as_deref()
        )
        .execute(&*self.database)
        .await?;
        Ok(())
    }

    pub async fn store_verification_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let token = Uuid::new_v4().to_string();
        let key = format!("email_verify:{}", token);
        let mut conn = self.redis.get().await.map_err(|error| {
            tracing::error!("Failed to get Redis connection: {:?}", error);
            AppError::Internal("Failed to create verification token".to_string())
        })?;
        redis::cmd("SET")
            .arg(&key)
            .arg(user_id.to_string())
            .arg("EX")
            .arg(86400u64)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|error| {
                tracing::error!("Failed to store token: {:?}", error);
                AppError::Internal("Failed to store verification token".to_string())
            })?;
        Ok(token)
    }

    pub async fn get_verification_token(&self, token: &str) -> Result<Option<String>, AppError> {
        let key = format!("email_verify:{}", token);
        let mut conn = self.redis.get().await.map_err(|e| {
            tracing::error!("Redis connection failed: {:?}", e);
            AppError::Internal("Redis connection failed".to_string())
        })?;
        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|error| {
                tracing::error!("Failed to get token: {:?}", error);
                AppError::Internal("Failed to fetch verification token".to_string())
            })
    }

    pub async fn delete_verification_token(&self, token: &str) -> Result<(), AppError> {
        let key = format!("email_verify:{}", token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("DEL")
            .arg(key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|error| {
                tracing::error!("Failed to get token: {:?}", error);
                AppError::Internal("Failed to fetch verification token".to_string())
            })
    }
}
