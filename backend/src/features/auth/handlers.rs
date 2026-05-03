use crate::common::utils::extract_ip_address;
use crate::features::auth::{RegisterRequest, UserResponse, VerifyEmailRequest};
use crate::{app::AppState, common::types::ApiResponse, errors::AppError};
use axum::{Json, extract::State, http::HeaderMap, http::StatusCode};
use std::sync::Arc;
use validator::Validate;

/// POST /api/v1/auth/register
pub async fn register(
    State(app): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), AppError> {
    body.validate()?;
    let ip = extract_ip_address(&headers);
    let user = app.auth_service.register(&app, body, ip).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(user))))
}

/// POST /api/v1/auth/verify-email
pub async fn verify_email(
    State(app): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<VerifyEmailRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let token = &body.token;
    let ip = extract_ip_address(&headers);
    app.auth_service.verify_email(token, ip).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(()))))
}
