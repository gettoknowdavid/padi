use crate::app::AppState;
use crate::errors::AppError;
use crate::features::auth::password::hash_password;
use crate::features::auth::repository::{create_audit_log, create_user, user_exists};
use crate::features::auth::{RegisterRequest, UserResponse};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        app: &AppState,
        body: RegisterRequest,
        ip: Option<String>,
    ) -> Result<UserResponse, AppError> {
        let db = app.db.clone();

        let exists = user_exists(&db, &body.email).await.map_err(|e| {
            tracing::error!("Database error while checking email: {:?}", e);
            AppError::Internal("Internal server error".to_string())
        })?;

        if exists {
            return Err(AppError::BadRequest(
                "A user with this email already exists".to_string(),
            ));
        }

        let hash_pwd = hash_password(&body.password).map_err(|e| {
            tracing::error!("Failed to hash password: {:?}", e);
            AppError::Internal("Failed to hash password".to_string())
        })?;

        let new_user = create_user(&db, &body, hash_pwd).await.map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            AppError::Internal("Failed to create user".to_string())
        })?;

        let verification_token = Uuid::new_v4().to_string();
        let redis_key = format!("email_verify:{}", verification_token);
        let user_id_str = new_user.id.to_string();

        let mut redis_conn = app.redis.get().await.map_err(|error| {
            tracing::error!("Failed to get Redis connection: {:?}", error);
            AppError::Internal("Failed to create verification token".to_string())
        })?;
        redis::cmd("SET")
            .arg(&redis_key)
            .arg(&user_id_str)
            .arg("EX")
            .arg(86400u64)
            .query_async::<()>(&mut redis_conn)
            .await
            .map_err(|error| {
                tracing::error!("Failed to store verification token in Redis: {:?}", error);
                AppError::Internal("Failed to create verification token".to_string())
            })?;

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

        if let Err(e) = create_audit_log(&db, new_user.id, "user.registered", ip).await {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(new_user.into_response())
    }
}
