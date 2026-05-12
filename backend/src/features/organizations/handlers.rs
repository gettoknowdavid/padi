use crate::app::AppState;
use crate::common::types::ApiResponse;
use crate::common::utils::ip_address::extract_ip_address;
use crate::errors::AppError;
use crate::features::auth::prelude::AuthUser;
use crate::features::auth::rbac::{Role, require_at_least};
use crate::features::organizations::dto::{
    CreateOrgRequest, CreateOrgResponse, InviteRequest, OrgResponse,
};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

// POST /api/v1/organizations
pub async fn create_org(
    Extension(auth): Extension<AuthUser>,
    State(app): State<Arc<AppState>>,
    Json(body): Json<CreateOrgRequest>,
) -> Result<(StatusCode, Json<ApiResponse<CreateOrgResponse>>), AppError> {
    let org = app.org_service.create_org(&auth, &body).await?;
    let access_token = app
        .auth_service
        .issue_org_token(&app, auth.user_id, org.id, "owner")?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::ok(CreateOrgResponse { org, access_token })),
    ))
}

// POST /api/v1/organizations/:id/invitations
pub async fn invite_member(
    Extension(auth): Extension<AuthUser>,
    State(app): State<Arc<AppState>>,
    Path(org_id): Path<Uuid>,
    headers: HeaderMap,
    Json(body): Json<InviteRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    require_at_least(&auth, Role::Admin)?;
    if Some(org_id) != auth.org_id {
        return Err(AppError::Forbidden);
    }
    let ip = extract_ip_address(&headers);
    app.org_service
        .invite_member(&app, &auth, &body, org_id, ip)
        .await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(()))))
}

// POST /api/v1/invitations/:token/accept
pub async fn accept_invite(
    Extension(auth): Extension<AuthUser>,
    State(app): State<AppState>,
    Path(token): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<OrgResponse>>), AppError> {
    let org = app.org_service.accept_invite(&auth, token).await?;
    Ok((StatusCode::OK, Json(ApiResponse::ok(org))))
}
