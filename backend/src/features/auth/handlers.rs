use crate::common::utils::extract_ip_address;
use crate::features::auth::service::AuthService;
use crate::features::auth::{RegisterRequest, UserResponse};
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
    let user = AuthService::register(&app, body, ip).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(user))))
}
