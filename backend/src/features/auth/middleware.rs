use crate::app::AppState;
use crate::errors::AppError;
use crate::features::auth::jwt::decode_access_token;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::errors::ErrorKind;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub org_id: Option<Uuid>,
    pub role: Option<String>,
}

pub async fn auth_middleware(
    State(app): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing auth token".into()))?;

    let claims =
        decode_access_token(token, &app.config.jwt_secret).map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired("Token expired".to_string()),
            _ => AppError::Unauthorized("Invalid token".to_string()),
        })?;

    req.extensions_mut().insert(AuthUser {
        user_id: claims.sub,
        org_id: claims.org_id,
        role: claims.role,
    });

    Ok(next.run(req).await)
}
