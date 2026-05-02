use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError};

// Requests
//
#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
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

// Responses
//
#[derive(Debug, Serialize, FromRow)]
pub struct User {
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
    pub user: User,
}

// Helper functions
fn validate_strong_password(password: &str) -> Result<(), ValidationError> {
    let mut errors: Vec<&str> = Vec::new();

    if password.len() < 12 {
        errors.push("Password must be at least 12 characters");
    }

    if password.len() > 128 {
        errors.push("Password is too long");
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        errors.push("Password must contain at least one lowercase letter");
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        errors.push("Password must contain at least one uppercase letter");
    }

    if errors.is_empty() {
        Ok(())
    } else {
        // Join all errors into one message (validator expects a single ValidationError)
        let message = errors.join(", ");
        Err(ValidationError::new("strong_password").with_message(std::borrow::Cow::from(message)))
    }
}
