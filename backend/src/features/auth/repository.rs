use crate::errors::AppError;
use crate::features::auth::RegisterRequest;
use crate::features::auth::models::User;
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

    pub async fn fetch_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.database)
        .await
        .map_err(|error| {
            tracing::error!("Error fetching user from DB: {:?}", error);
            AppError::Internal("Error fetching user from DB".to_string())
        })
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
    ) -> Result<(), AppError> {
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
        .await
        .map_err(|e| {
            tracing::error!("Failed to write audit log: {:?}", e);
            AppError::Internal("Failed to write audit log".to_string())
        })?;
        Ok(())
    }

    pub async fn update_login_attempt(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = failed_login_attempts + 1 WHERE id = $1",
            user_id
        )
        .execute(&*self.database)
        .await?;
        Ok(())
    }

    pub async fn reset_login_attempts_and_lock(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = $1",
            user_id
        )
        .execute(&*self.database)
        .await?;
        Ok(())
    }

    pub async fn lock_user_account(&self, user_id: Uuid) -> Result<(), Error> {
        let now = chrono::Utc::now();
        let locked_until_value = now + chrono::Duration::hours(1);
        sqlx::query!(
            r#"UPDATE users SET locked_until = $1 WHERE id = $2"#,
            locked_until_value,
            user_id
        )
        .execute(&*self.database)
        .await?;
        Ok(())
    }

    pub async fn update_password_hash(&self, user_id: &Uuid, hash: &str) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1, failed_login_attempts = 0, locked_until = NULL WHERE id = $2",
            hash,
            user_id
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

    pub async fn delete_refresh_token(&self, token: &str) -> Result<(), AppError> {
        let key = format!("refresh_token:{}", token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete refresh token: {:?}", e);
                AppError::Internal("Failed to delete refresh token".to_string())
            })
    }

    pub async fn revoke_refresh_tokens(&self, user_id: &str) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;

        let keys: Vec<String> = redis::cmd("SCAN")
            .arg(0u64)
            .arg("MATCH")
            .arg("refresh_token:*")
            .arg("COUNT")
            .arg(100u64)
            .query_async(&mut conn)
            .await
            .map_err(|e| {
                tracing::error!("Failed to scan Redis keys: {:?}", e);
                AppError::Internal("Failed to scan tokens".to_string())
            })?;

        for key in keys {
            // GET the value (user_id stored against this token)
            let value: Option<String> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            // Only delete if it belongs to this user
            if value.as_deref() == Some(user_id) {
                let _ = redis::cmd("DEL")
                    .arg(&key)
                    .query_async::<()>(&mut conn)
                    .await;
            }
        }

        Ok(())
    }
}
