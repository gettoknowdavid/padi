use validator::ValidationError;

pub fn validate_strong_password(password: &str) -> Result<(), ValidationError> {
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
