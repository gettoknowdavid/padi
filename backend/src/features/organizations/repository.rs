use crate::common::utils::slug::slugify;
use crate::errors::AppError;
use crate::features::organizations::models::{Invitation, Organization};
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct OrgRepository {
    database: Arc<sqlx::PgPool>,
}

impl OrgRepository {
    pub fn new(database: Arc<sqlx::PgPool>) -> Self {
        Self { database }
    }

    // ── Database queries ─────────────────────────────────────────
    pub async fn create_org(
        &self,
        name: &str,
        slug: &str,
        owner_id: &Uuid,
    ) -> Result<Organization, AppError> {
        let mut tx = self.database.begin().await.map_err(AppError::Database)?;

        // Create a new organization
        let org = sqlx::query_as!(
            Organization,
            r#"INSERT INTO organizations (id, name, slug) VALUES ($1, $2, $3) RETURNING *"#,
            Uuid::new_v4(),
            name,
            slug
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Create the organization owner
        sqlx::query_scalar!(
            r#"INSERT INTO organization_members (org_id, user_id, role) VALUES ($1, $2, 'owner')"#,
            &org.id,
            owner_id,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Commit the transaction
        tx.commit().await.map_err(AppError::Database)?;

        Ok(org)
    }
    pub async fn find_org_by_id(&self, id: Uuid) -> Result<Option<Organization>, AppError> {
        sqlx::query_as!(
            Organization,
            r#"SELECT * FROM organizations WHERE id = $1"#,
            id
        )
        .fetch_optional(&*self.database)
        .await
        .map_err(AppError::Database)
    }
    pub async fn create_invite(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        email: String,
        role: String,
        token: String,
        invited_by: Uuid,
        ip: Option<String>,
    ) -> Result<(), AppError> {
        let mut tx = self.database.begin().await.map_err(AppError::Database)?;

        sqlx::query!(
            r#"
            INSERT INTO invitations (id, org_id, email, role, token, invited_by, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            Uuid::new_v4(),
            org_id,
            email,
            role,
            token,
            invited_by,
            Utc::now() + Duration::days(7)
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        sqlx::query!(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, ip_address)
            VALUES ($1, $2, $3, 'org', $4, $5)
            "#,
            Uuid::new_v4(),
            user_id,
            "org_invite_sent",
            user_id,
            ip.as_deref()
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
    pub async fn find_invitation(&self, token: String) -> Result<Option<Invitation>, AppError> {
        sqlx::query_as!(
            Invitation,
            r#"SELECT * FROM invitations WHERE token = $1 AND accepted_at IS NULL AND expires_at > NOW() LIMIT 1"#,
            token
        )
            .fetch_optional(&*self.database)
            .await
            .map_err(AppError::Database)
    }
    pub async fn accept_invitation(
        &self,
        token: String,
        user_id: Uuid,
    ) -> Result<(Organization, String), AppError> {
        let mut tx = self.database.begin().await.map_err(AppError::Database)?;

        let invitation = sqlx::query_as!(
            Invitation,
            r#"
            SELECT * FROM invitations
            WHERE token = $1
              AND accepted_at IS NULL
              AND expires_at > NOW()
            LIMIT 1
            "#,
            token
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let invitation = invitation
            .ok_or_else(|| AppError::NotFound("Invite expired or already used".to_string()))?;

        let role = invitation.role.clone();

        sqlx::query!(
            r#"
            INSERT INTO organization_members (org_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (org_id, user_id) DO NOTHING
            "#,
            invitation.org_id,
            user_id,
            &role,
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        sqlx::query!(
            r#"UPDATE invitations SET accepted_at = NOW() WHERE token = $1"#,
            token
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let org = sqlx::query_as!(
            Organization,
            r#"SELECT  * FROM organizations WHERE id = $1"#,
            invitation.org_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok((org, role))
    }
    pub async fn slug_exists(&self, slug: &str) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM organizations WHERE slug = $1) as "exists!""#,
            slug
        )
        .fetch_one(&*self.database)
        .await
        .map_err(AppError::Database)
    }
    pub async fn create_unique_slug(&self, name: &str) -> Result<String, AppError> {
        let mut slug = slugify(name);

        if !self.slug_exists(&slug).await? {
            return Ok(slug);
        }

        let mut counter = 1;
        const MAX_ATTEMPTS: u32 = 200;

        while counter <= MAX_ATTEMPTS {
            slug = format!("{}-{}", slug, counter);
            if !self.slug_exists(&slug).await? {
                return Ok(slug);
            }
            counter += 1;
        }

        // Extremely rare case — fall back to UUID suffix
        let fallback = format!("{}-{}", slug, Uuid::new_v4().simple());
        tracing::warn!("Used UUID fallback for slug base: {}", slug);

        Ok(fallback)
    }
}
