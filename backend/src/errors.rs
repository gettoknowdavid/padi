use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound(String),

    #[error("You are unauthorized")]
    Unauthorized(String),

    #[error("Bad request")]
    BadRequest(String),

    #[error("Internal error")]
    Internal(String),

    #[error("Validation error")]
    ValidatorError(ValidationErrors),

    #[error("Conflict")]
    Conflict(String),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::ValidatorError(errors)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::NotFound(message) => {
                error_response(StatusCode::NOT_FOUND, "NOT_FOUND", message)
            }
            AppError::Unauthorized(message) => {
                error_response(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", message)
            }
            AppError::BadRequest(message) => {
                error_response(StatusCode::BAD_REQUEST, "BAD_REQUEST", message)
            }
            AppError::Internal(message) => {
                error_response(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", message)
            }
            AppError::Conflict(message) => {
                error_response(StatusCode::CONFLICT, "CONFLICT", message)
            }
            AppError::ValidatorError(validation_errors) => {
                validation_error_response(validation_errors.clone())
            }
        }
    }
}

fn error_response(status: StatusCode, code: &str, message: &str) -> Response {
    let body = Json(json!({
        "data": null,
        "meta": null,
        "error": {
            "code": code,
            "message": message,
        }
    }));

    (status, body).into_response()
}

fn validation_error_response(validation_errors: ValidationErrors) -> Response {
    // Convert ValidationErrors into a nice format for the frontend
    let error_map: std::collections::HashMap<String, Vec<String>> = validation_errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|err| {
                    err.message
                        .as_deref()
                        .unwrap_or("Invalid value")
                        .to_string()
                })
                .collect();
            (field.to_string(), messages)
        })
        .collect();

    let body = Json(json!({
        "data": null,
        "meta": null,
        "error": {
            "code": "VALIDATION_ERROR",
            "message": "One or more fields are invalid",
            "fields": error_map
        }
    }));

    (StatusCode::BAD_REQUEST, body).into_response()
}
