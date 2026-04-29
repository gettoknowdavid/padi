use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("You are unauthorized")]
    Unauthorized,

    #[error("Bad request")]
    BadRequest,

    #[error("Internal error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "data": null,
            "meta": null,
            "error": {
                "message": self.to_string(),
            }
        }));

        (status, body).into_response()
    }
}
