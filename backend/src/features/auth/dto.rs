use crate::features::auth::validators::{validate_email_format, validate_strong_password};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// Requests
#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(custom(function = "validate_email_format"))]
    pub email: String,

    #[validate(custom(function = "validate_strong_password"))]
    pub password: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Full name must be between 1 and 50 characters"
    ))]
    pub full_name: String,

    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

// Responses
#[derive(Debug, Serialize, FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub full_name: String,
    pub is_verified: bool,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}
