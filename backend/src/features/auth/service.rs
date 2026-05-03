use crate::app::AppState;
use crate::errors::AppError;
use crate::features::auth::password::hash_password;
use crate::features::auth::repository::AuthRepository;
use crate::features::auth::{RegisterRequest, UserResponse};
use anyhow::Result;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(database: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
        Self {
            repo: AuthRepository::new(database, redis),
        }
    }

    pub async fn register(
        &self,
        app: &AppState,
        body: RegisterRequest,
        ip: Option<String>,
    ) -> Result<UserResponse, AppError> {
        if self.repo.user_exists(&body.email).await? {
            return Err(AppError::BadRequest(
                "A user with this email already exists".to_string(),
            ));
        }

        let hash_pwd = hash_password(&body.password).map_err(|e| {
            tracing::error!("Failed to hash password: {:?}", e);
            AppError::Internal("Failed to hash password".to_string())
        })?;

        let new_user = self.repo.create_user(&body, hash_pwd).await?;

        let verification_token = self.repo.store_verification_token(new_user.id).await?;

        let verification_link = format!(
            "{}/auth/verify-email?token={}",
            app.config.frontend_url, verification_token
        );

        let email_body = serde_json::json!({
            "subject": "Confirm your Padi account",
            "from": "Padi <noreply@yourpadiapp.com>",
            "to":[&body.email],
            "html": format!(
               "<p>Hi {},</p><p>Click the link below to verify your account:</p><p><a href='{}'>Verify Email</a></p><p>This link expires in 24 hours.</p>",
                &body.full_name, verification_link
            ),
        });

        let header_auth = format!("Bearer {}", app.config.resend_api_key);

        let email_result = app
            .http_client
            .post("https://api.resend.com/emails")
            .header("Authorization", header_auth)
            .header("Content-Type", "application/json")
            .json(&email_body)
            .send()
            .await;

        if let Err(error) = email_result {
            // Non-critical — user is created, just log the failure
            tracing::error!("Failed to send verification email: {:?}", error);
        }

        if let Err(e) = self
            .repo
            .create_audit_log(new_user.id, "user.registered", ip)
            .await
        {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(new_user.into_response())
    }

    pub async fn verify_email(&self, token: &str, ip: Option<String>) -> Result<(), AppError> {
        let verification_token = self.repo.get_verification_token(token).await?;

        let user_id_str = verification_token
            .ok_or_else(|| AppError::BadRequest("Invalid or expired token".to_string()))?;

        let user_id = Uuid::parse_str(&user_id_str).map_err(|error| {
            tracing::error!("Failed to parse user ID from token: {:?}", error);
            AppError::Internal("Invalid token payload".to_string())
        })?;

        self.repo.set_user_verified(&user_id).await?;

        self.repo.delete_verification_token(token).await?;

        if let Err(e) = self
            .repo
            .create_audit_log(user_id, "user.verified", ip)
            .await
        {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(())
    }
}
