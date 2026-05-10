use crate::common::domain::phone_number::PhoneNumber;
use crate::errors::AppError;
use crate::features::auth::constants::{ACCOUNT_LOCK_HOURS, EMAIL_VERIFY_TTL_SECS, OTP_RATE_LIMIT_MAX, OTP_TTL_SECS, PWD_RESET_TTL_SECS, RATE_LIMIT_WINDOW_SECONDS, REDIS_EMAIL_VERIFY_PREFIX, REDIS_OTP_ATTEMPT_PREFIX, REDIS_OTP_PREFIX, REDIS_OTP_RATE_LIMIT_PREFIX, REDIS_PWD_RESET_PREFIX, REDIS_REFRESH_TOKEN_PREFIX, REFRESH_TOKEN_TTL_SECS};
use crate::features::auth::dto::RegisterRequest;
use crate::features::auth::models::User;
use deadpool_redis::{Connection, Pool as RedisPool};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthRepository {
    pub database: Arc<PgPool>,
    pub redis: Arc<RedisPool>,
}

impl AuthRepository {
    pub fn new(database: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
        Self { database, redis }
    }

    async fn redis_conn(&self) -> Result<Connection, AppError> {
        self.redis.get().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::RedisPool(e)
        })
    }

    // ── User queries ─────────────────────────────────────────
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_optional(&*self.database)
            .await
            .map_err(AppError::Database)
    }
    pub async fn find_by_phone(&self, phone: &PhoneNumber) -> Result<Option<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE phone = $1"#, phone.e164)
            .fetch_optional(&*self.database)
            .await
            .map_err(AppError::Database)
    }
    pub async fn exists(&self, email: &str) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!""#,
            email
        )
        .fetch_one(&*self.database)
        .await
        .map_err(AppError::Database)
    }
    pub async fn create(&self, req: &RegisterRequest, hash: String) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, full_name, phone, is_verified)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &req.email,
            hash,
            req.full_name.as_str(),
            req.phone.as_deref(),
            false
        )
        .fetch_one(&*self.database)
        .await
        .map_err(AppError::Database)
    }
    pub async fn create_phone_user(&self, phone: &PhoneNumber) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, phone, is_verified, failed_login_attempts)
            VALUES ($1, $2, true, 0)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &phone.e164,
        )
        .fetch_one(&*self.database)
        .await
        .map_err(AppError::Database)
    }
    pub async fn set_verified(&self, user_id: &Uuid) -> Result<(), AppError> {
        sqlx::query!("UPDATE users SET is_verified = true WHERE id = $1", user_id)
            .execute(&*self.database)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
    pub async fn update_password(&self, user_id: &Uuid, hash: &str) -> Result<(), AppError> {
        sqlx::query!(
            r#"UPDATE users SET password_hash = $1, failed_login_attempts = 0, locked_until = NULL WHERE id = $2"#,
            hash,
            user_id
        )
            .execute(&*self.database)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
    pub async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = failed_login_attempts + 1 WHERE id = $1",
            user_id
        )
        .execute(&*self.database)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }
    pub async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = $1",
            user_id
        )
        .execute(&*self.database)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }
    pub async fn lock_account(&self, user_id: Uuid) -> Result<(), AppError> {
        let locked_until = chrono::Utc::now() + chrono::Duration::hours(ACCOUNT_LOCK_HOURS);
        sqlx::query!(
            r#"UPDATE users SET locked_until = $1 WHERE id = $2"#,
            locked_until,
            user_id
        )
        .execute(&*self.database)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    // ── Audit queries ─────────────────────────────────────────
    pub async fn audit(
        &self,
        user_id: Uuid,
        action: &str,
        ip: Option<String>,
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
            ip.as_deref()
        )
            .execute(&*self.database)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    // ── Email verification tokens (Redis) ────────────────────
    pub async fn store_verification_token(&self, user_id: &Uuid) -> Result<String, AppError> {
        let token = Uuid::new_v4().to_string();
        let key = format!("{}{}", REDIS_EMAIL_VERIFY_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("SET")
            .arg(&key)
            .arg(user_id.to_string())
            .arg("EX")
            .arg(EMAIL_VERIFY_TTL_SECS)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)?;
        Ok(token)
    }
    pub async fn get_verification_token(&self, token: &str) -> Result<Option<String>, AppError> {
        let key = format!("{}{}", REDIS_EMAIL_VERIFY_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn delete_verification_token(&self, token: &str) -> Result<(), AppError> {
        let key = format!("{}{}", REDIS_EMAIL_VERIFY_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("DEL")
            .arg(key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)
    }

    // ── Password reset tokens (Redis) ────────────────────────
    pub async fn store_reset_token(&self, user_id: &str) -> Result<String, AppError> {
        let token = Uuid::new_v4().to_string();
        let key = format!("{}{}", REDIS_PWD_RESET_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("SET")
            .arg(&key)
            .arg(user_id)
            .arg("EX")
            .arg(PWD_RESET_TTL_SECS)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)?;
        Ok(token)
    }
    pub async fn get_reset_token(&self, token: &str) -> Result<Option<String>, AppError> {
        let key = format!("{}{}", REDIS_PWD_RESET_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn delete_reset_token(&self, token: &str) -> Result<(), AppError> {
        let key = format!("{}{}", REDIS_PWD_RESET_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)
    }

    // ── Session/Refresh tokens (Redis) ─────────────────────
    pub async fn create_session(&self, user_id: &str) -> Result<String, AppError> {
        let token = Uuid::new_v4().to_string();
        let key = format!("{}{}", REDIS_REFRESH_TOKEN_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("SET")
            .arg(&key)
            .arg(user_id)
            .arg("EX")
            .arg(REFRESH_TOKEN_TTL_SECS)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)?;
        Ok(token)
    }
    pub async fn delete_session(&self, token: &str) -> Result<(), AppError> {
        let key = format!("{}{}", REDIS_REFRESH_TOKEN_PREFIX, token);
        let mut conn = self.redis_conn().await?;
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn _rotate_session(&self, old_token: &str) -> Result<(String, String), AppError> {
        let key = format!("{}{}", REDIS_REFRESH_TOKEN_PREFIX, old_token);
        let mut conn = self.redis_conn().await?;

        let user_id: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)?;

        let user_id = user_id.ok_or_else(|| {
            AppError::Unauthorized("Invalid or expired refresh token".to_string())
        })?;

        // Delete old token
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)?;

        // Create new token
        let new_token = self.create_session(&user_id).await?;
        Ok((user_id, new_token))
    }
    pub async fn _revoke_all_sessions(&self, user_id: &str) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;
        let pattern = format!("{}*", REDIS_REFRESH_TOKEN_PREFIX);

        let keys: Vec<String> = redis::cmd("SCAN")
            .arg(0u64)
            .arg("MATCH")
            .arg(&pattern)
            .arg("COUNT")
            .arg(100u64)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)?;

        for key in keys {
            let value: Option<String> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .unwrap_or(None);

            if value.as_deref() == Some(user_id) {
                let _ = redis::cmd("DEL")
                    .arg(&key)
                    .query_async::<()>(&mut conn)
                    .await;
            }
        }

        Ok(())
    }

    // ── Rate Limiting for Phone OTP (Redis) ─────────────────────
    pub async fn check_rate_limit(&self, phone: &PhoneNumber) -> Result<bool, AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_RATE_LIMIT_PREFIX, &phone.e164);

        let script = redis::Script::new(
            r#"
            local count = redis.call('INCR', KEYS[1])
            if count == 1 then
                redis.call('EXPIRE', KEYS[1], ARGV[1])
            end
            return count
        "#,
        );

        let count: i64 = script
            .key(&key)
            .arg(RATE_LIMIT_WINDOW_SECONDS)
            .invoke_async(&mut conn)
            .await
            .map_err(AppError::Redis)?;

        Ok(count <= OTP_RATE_LIMIT_MAX)
    }
    pub async fn store_phone_otp(&self, phone: &PhoneNumber, otp: &str) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_PREFIX, &phone.e164);

        redis::cmd("SET")
            .arg(&key)
            .arg(otp)
            .arg("EX")
            .arg(OTP_TTL_SECS)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn increment_otp_attempts(&self, phone: &PhoneNumber) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_ATTEMPT_PREFIX, &phone.e164);
        redis::cmd("INCR")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn check_otp_attempts(&self, phone: &PhoneNumber) -> Result<Option<u32>, AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_ATTEMPT_PREFIX, &phone.e164);
        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn reset_otp_attempts(&self, phone: &PhoneNumber) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_ATTEMPT_PREFIX, &phone.e164);
        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn get_otp(&self, phone: &PhoneNumber) -> Result<Option<String>, AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_PREFIX, &phone.e164);
        redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
    pub async fn delete_otp(&self, phone: &PhoneNumber) -> Result<(), AppError> {
        let mut conn = self.redis_conn().await?;
        let key = format!("{}{}", REDIS_OTP_PREFIX, &phone.e164);
        redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(AppError::Redis)
    }
}
