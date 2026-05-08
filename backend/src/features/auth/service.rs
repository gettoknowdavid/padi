use crate::app::AppState;
use crate::common::domain::phone_number::PhoneNumber;
use crate::errors::AppError;
use crate::features::auth::dto::{SendOtpRequest, VerifyOtpRequest};
use crate::features::auth::jwt::{CreateTokenArgs, create_access_token};
use crate::features::auth::password::{hash_password, verify_password};
use crate::features::auth::prelude::{
    AuthResponse, LoginRequest, RegisterRequest, ResetPasswordRequest, UserResponse,
};
use crate::features::auth::repository::AuthRepository;
use crate::features::auth::utils::generate_otp;
use crate::integrations::termii::send_otp;
use anyhow::Result;
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(database: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
        Self {
            repo: AuthRepository::new(database, redis),
        }
    }

    /// Registers a new user by validating input, storing user details in the database,
    /// hashing the password, sending a verification email, and logging the action.
    ///
    /// # Arguments
    ///
    /// * `app` - A reference to the shared application state (`AppState`) containing configuration and dependencies.
    /// * `body` - The registration request payload containing user details (`RegisterRequest`).
    /// * `ip` - Optional IP address of the client initiating the request, used for audit logging.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `UserResponse` on success, which represents the newly created user's response model.
    /// - `AppError` on failure, detailing the specific error encountered.
    ///
    /// # Errors
    ///
    /// This function can return the following errors:
    /// - `AppError::BadRequest`: If a user with the provided email already exists.
    /// - `AppError::Internal`: If password hashing fails or due to other unexpected internal issues.
    ///
    /// # Process
    ///
    /// 1. Checks if a user with the provided email already exists in the repository.
    /// 2. Hashes the user-provided password securely.
    /// 3. Creates a new user record in the database using the hashed password and input data.
    /// 4. Generates an email verification token for the new user.
    /// 5. Sends a verification email containing a link to confirm the user's email address.
    /// 6. Attempts to log the registration event in the audit log, although this step is non-critical.
    ///
    /// # Note
    ///
    /// - If sending the verification email fails, the user is still registered, but the error is logged.
    /// - Audit logging of the registration, if it fails, does not affect the overall success of the registration process.
    ///
    /// # Example
    ///
    /// ```rust
    /// let app_state = AppState::new();
    /// let registration_request = RegisterRequest {
    ///     email: "user@example.com".to_string(),
    ///     full_name: "John Doe".to_string(),
    ///     password: "secure_password".to_string(),
    /// };
    ///
    /// let result = user_service
    ///     .register(&app_state, registration_request, Some("127.0.0.1".to_string()))
    ///     .await;
    ///
    /// match result {
    ///     Ok(user_response) => println!("User registered successfully: {:?}", user_response),
    ///     Err(err) => println!("Registration failed: {:?}", err),
    /// }
    /// ```
    pub async fn register(
        &self,
        app: &AppState,
        body: &RegisterRequest,
        ip: Option<String>,
    ) -> Result<UserResponse, AppError> {
        if self.repo.exists(&body.email).await? {
            return Err(AppError::BadRequest(
                "A user with this email already exists".to_string(),
            ));
        }
        let hash_pwd = hash_password(&body.password).map_err(|e| {
            tracing::error!("Failed to hash password: {:?}", e);
            AppError::Internal("Failed to hash password".to_string())
        })?;
        let new_user = self.repo.create(body, hash_pwd).await?;
        let verification_token = self.repo.store_verification_token(&new_user.id).await?;

        let verification_link = format!(
            "{}/auth/verify-email?token={}",
            app.config.frontend_url, verification_token
        );

        app.email.send(&body.email, "Confirm your Padi account", &format!(
            "<p>Hi {},</p><p>Click the link below to verify your account:</p><p><a href='{}'>Verify Email</a></p><p>This link expires in 24 hours.</p>",
            &body.full_name, verification_link
        )).await;

        if let Err(e) = self.repo.audit(new_user.id, "user.registered", ip).await {
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

        self.repo.set_verified(&user_id).await?;

        self.repo.delete_verification_token(token).await?;

        if let Err(e) = self.repo.audit(user_id, "user.verified", ip).await {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(())
    }

    pub async fn login(
        &self,
        app: &AppState,
        body: &LoginRequest,
        ip: Option<String>,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .repo
            .find_by_email(&body.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        if !user.is_verified {
            return Err(AppError::Unauthorized(
                "Please verify your email first".to_string(),
            ));
        }

        if let Some(locked_until) = user.locked_until {
            if locked_until > Utc::now() {
                return Err(AppError::Unauthorized(
                    "Account is locked. Try again later.".to_string(),
                ));
            }
        }

        let password_hash = user.password_hash.as_ref().ok_or_else(|| {
            tracing::error!("Password hash is missing for user: {:?}", user.id);
            AppError::Internal("This account uses a different login method".to_string())
        })?;

        if !verify_password(&body.password, password_hash) {
            let new_attempts = user.failed_login_attempts + 1;

            if let Err(e) = self.repo.increment_failed_attempts(user.id).await {
                tracing::error!("Failed to update login attempts: {:?}", e);
            };

            if new_attempts >= 10 {
                if let Err(e) = self.repo.lock_account(user.id).await {
                    tracing::error!("Failed to lock user account: {:?}", e);
                };
                return Err(AppError::Unauthorized(
                    "Account locked due to too many failed attempts.".to_string(),
                ));
            }

            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        if let Err(e) = self.repo.reset_failed_attempts(user.id).await {
            tracing::error!("Failed to update failed login attempts: {:?}", e);
        };

        let user_id = user.id.to_string();

        let access_token = create_access_token(CreateTokenArgs {
            user_id: user_id.clone(),
            secret: app.config.jwt_secret.to_string(),
            org_id: None,
            role: None,
        })
        .map_err(|error| {
            tracing::error!("Failed to create access token: {:?}", error);
            AppError::Internal("Failed to create access token".to_string())
        })?;

        let refresh_token = self.repo.create_session(&user_id).await?;

        if let Err(e) = self.repo.audit(user.id, "user.login", ip).await {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: user.into_response(),
        })
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), AppError> {
        self.repo.delete_session(refresh_token).await
    }

    pub async fn forgot_password(&self, app: &AppState, email: &str) -> Result<(), AppError> {
        let user_option = self.repo.find_by_email(email).await?;

        if let Some(user) = user_option {
            let pwd_token = self.repo.store_reset_token(&user.id.to_string()).await?;

            let reset_link = format!(
                "{}/auth/reset-password?token={}",
                &app.config.frontend_url, pwd_token
            );

            app.email.send(email, "Password Reset Request", &format!(
                "<p>Click the link below to reset the password to your account:</p><p><a href='{}'>Password Reset</a></p><p>This link expires in 1 hour.</p>",
                reset_link
            )).await;
        }

        Ok(())
    }

    pub async fn reset_password(
        &self,
        body: &ResetPasswordRequest,
        ip: Option<String>,
    ) -> Result<(), AppError> {
        body.validate()?;

        let value = self
            .repo
            .get_reset_token(&body.token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired token".to_string()))?;

        let user_id = Uuid::parse_str(&value).map_err(|e| {
            tracing::error!("Failed to parse user ID: {:?}", e);
            AppError::Internal("Invalid user ID".to_string())
        })?;

        let hash = hash_password(&body.new_password).map_err(|e| {
            tracing::error!("Failed to hash password: {:?}", e);
            AppError::Internal("Failed to hash password".to_string())
        })?;

        self.repo.update_password(&user_id, &hash).await?;

        self.repo.delete_reset_token(&body.token).await?;

        if let Err(e) = self.repo.audit(user_id, "user.pwd_reset", ip).await {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(())
    }

    pub async fn send_otp(&self, app: &AppState, body: &SendOtpRequest) -> Result<(), AppError> {
        let phone_number = PhoneNumber::normalize(&body.phone, body.country_code.as_deref())
            .map_err(|_| AppError::BadRequest("Invalid phone number".to_string()))?;

        let within_limit = self.repo.check_rate_limit(&phone_number).await?;

        if !within_limit {
            return Err(AppError::TooManyRequests(
                "Too many OTP requests. Please try again later.".to_string(),
            ));
        }

        let otp = generate_otp();

        self.repo.store_phone_otp(&phone_number, &otp).await?;

        self.repo.reset_otp_attempts(&phone_number).await?;

        send_otp(
            &app.http_client,
            &app.config.africas_talking_api_key,
            &phone_number.e164.as_str(),
            otp.as_str(),
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to send OTP: {:?}", e);
            AppError::Internal("Failed to send OTP".to_string())
        })?;

        Ok(())
    }

    pub async fn verify_otp(
        &self,
        app: &AppState,
        body: &VerifyOtpRequest,
        ip: Option<String>,
    ) -> Result<AuthResponse, AppError> {
        let phone_number = PhoneNumber::normalize(&body.phone, body.country_code.as_deref())
            .map_err(|_| AppError::BadRequest("Invalid phone number".to_string()))?;

        self.repo.increment_otp_attempts(&phone_number).await?;

        if let Some(attempts) = self.repo.check_otp_attempts(&phone_number).await? {
            if attempts >= 3 {
                return Err(AppError::TooManyRequests(
                    "Too many OTP attempts. Please try again later.".to_string(),
                ));
            };
        }

        let otp = self.repo.get_otp(&phone_number).await?;

        if otp.is_none() {
            return Err(AppError::Unauthorized("Invalid or expired OTP".to_string()));
        }

        if otp.as_deref() != Some(&body.otp) {
            return Err(AppError::Unauthorized("Invalid OTP".to_string()));
        }

        self.repo.delete_otp(&phone_number).await?;

        let user = match self.repo.find_by_phone(&phone_number).await? {
            Some(existing) => existing,
            None => self.repo.create_phone_user(&phone_number).await?, // new user
        };

        let access_token = create_access_token(CreateTokenArgs {
            user_id: user.id.to_string(),
            secret: app.config.jwt_secret.to_string(),
            org_id: None,
            role: None,
        })
        .map_err(|error| {
            tracing::error!("Failed to create access token: {:?}", error);
            AppError::Internal("Failed to create access token".to_string())
        })?;

        let refresh_token = self.repo.create_session(&user.id.to_string()).await?;

        if let Err(e) = self.repo.audit(user.id, "user.otp_verified", ip).await {
            tracing::error!("Failed to write audit log: {:?}", e);
        };

        Ok(AuthResponse {
            access_token,
            refresh_token,
            user: user.into_response(),
        })
    }
}
