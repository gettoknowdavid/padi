use crate::app::AppState;
use crate::errors::AppError;
use crate::features::auth::prelude::AuthUser;
use crate::features::auth::rbac::{Role, require_at_least};
use crate::features::organizations::dto::{CreateOrgRequest, InviteRequest, OrgResponse};
use crate::features::organizations::repository::OrgRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct OrgService {
    repo: OrgRepository,
}

impl OrgService {
    pub fn new(database: Arc<sqlx::PgPool>) -> Self {
        Self {
            repo: OrgRepository::new(database),
        }
    }

    pub async fn create_org(
        &self,
        auth: &AuthUser,
        req: &CreateOrgRequest,
    ) -> Result<OrgResponse, AppError> {
        let slug =
            self.repo.create_unique_slug(&req.name).await.map_err(|e| {
                AppError::Internal(format!("Failed to create unique slug: {:?}", e))
            })?;

        let org = self
            .repo
            .create_org(&req.name, &slug, &auth.user_id)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create organization: {:?}", e)))?;

        Ok(org.into_response())
    }
    pub async fn invite_member(
        &self,
        app: &AppState,
        auth: &AuthUser,
        req: &InviteRequest,
        org_id: Uuid,
        ip: Option<String>,
    ) -> Result<(), AppError> {
        require_at_least(&auth, Role::Admin)?;

        if Some(org_id) != auth.org_id {
            return Err(AppError::Forbidden);
        }

        let token = Uuid::new_v4().to_string();

        self.repo
            .create_invite(
                auth.user_id,
                org_id.clone(),
                req.email.clone(),
                req.role.clone(),
                token.clone(),
                auth.user_id.clone(),
                ip,
            )
            .await?;

        let mut organization_name = "Organization".to_string();

        if let Some(org) = self.repo.find_org_by_id(org_id).await? {
            organization_name = org.name;
        }

        let invite_link = format!("{}/accept-invite?token={}", app.config.frontend_url, token);

        let email_body = format!(
            r#"
            <p>You've been invited to join <strong>{}</strong> on Padi CRM.</p>
            <p>Click the link below to accept the invitation:</p>
            <p><a href="{}">Accept Invitation</a></p>
            <p>This link expires in 7 days.</p>
            "#,
            organization_name, invite_link
        );

        app.email
            .send(
                req.email.as_str(),
                "You've been invited to Padi CRM",
                &email_body,
            )
            .await;

        Ok(())
    }
    pub async fn accept_invite(
        &self,
        auth: &AuthUser,
        token: String,
    ) -> Result<OrgResponse, AppError> {
        let (org, _) = self.repo.accept_invitation(token, auth.user_id).await?;
        Ok(org.into_response())
    }
}
