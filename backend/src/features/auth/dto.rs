use crate::common::domain::phone_number::{CountryCode, PhoneNumber};
use crate::features::auth::validators::{
    validate_email_format, validate_phone, validate_strong_password,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// Helpers
mod lowercase_string {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.to_lowercase())
    }
}

// Requests
#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[serde(deserialize_with = "lowercase_string::deserialize")]
    #[validate(custom(function = "validate_email_format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ForgotPasswordRequest {
    #[serde(deserialize_with = "lowercase_string::deserialize")]
    #[validate(custom(function = "validate_email_format"))]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,

    #[validate(custom(function = "validate_strong_password"))]
    pub new_password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequest {
    #[serde(deserialize_with = "lowercase_string::deserialize")]
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

    #[validate(custom(function = "validate_phone"))]
    pub phone: String,

    pub phone_country: CountryCode,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Deserialize)]
pub struct SendOtpRequest {
    pub phone: PhoneNumber,
}

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub phone: PhoneNumber,
    pub otp: String,
}

// Responses
#[must_use]
#[derive(Debug, Serialize, FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub full_name: String,
    pub is_verified: bool,
}

#[must_use]
#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}
