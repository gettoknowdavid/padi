use crate::common::utils::extract_ip_address;
use crate::features::auth::prelude::{
    AuthResponse, ForgotPasswordRequest, LoginRequest, LogoutRequest, RegisterRequest,
    ResetPasswordRequest, UserResponse, VerifyEmailRequest,
};
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
    let user = app.auth_service.register(&app, &body, ip).await?;
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

/// POST /api/v1/auth/login
pub async fn login(
    State(app): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<AuthResponse>>), AppError> {
    body.validate()?;
    let ip = extract_ip_address(&headers);
    let user = app.auth_service.login(&app, &body, ip).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(user))))
}

/// POST /api/v1/auth/logout
pub async fn logout(
    State(app): State<Arc<AppState>>,
    Json(body): Json<LogoutRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    app.auth_service.logout(&body.refresh_token).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(()))))
}

/// POST /api/v1/auth/forgot-password
pub async fn forgot_password(
    State(app): State<Arc<AppState>>,
    Json(body): Json<ForgotPasswordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    body.validate()?;
    app.auth_service.forgot_password(&app, &body.email).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(()))))
}

/// POST /api/v1/auth/reset-password
pub async fn reset_password(
    State(app): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let ip = extract_ip_address(&headers);
    app.auth_service.reset_password(&body, ip).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(()))))
}
