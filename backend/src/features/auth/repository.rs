use crate::features::auth::models::User;
use crate::features::auth::{RegisterRequest, UserResponse};
use anyhow::Result;
use sqlx::{Error, PgPool, query, query_as, query_scalar};
use uuid::Uuid;

pub(super) async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<UserResponse>> {
    let user = query_as!(
        UserResponse,
        r#"
        SELECT id, email, phone, full_name, is_verified
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}

pub(super) async fn user_exists(db: &PgPool, email: &str) -> Result<bool, Error> {
    let exists = query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!""#,
        email
    )
    .fetch_one(db)
    .await?;
    Ok(exists)
}

pub(super) async fn create_user(
    db: &PgPool,
    req: &RegisterRequest,
    password_hash: String,
) -> Result<User, Error> {
    let new_user = query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password_hash, full_name, phone, is_verified)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, phone, full_name, avatar_url, password_hash, is_verified,
                  failed_login_attempts, locked_until, created_at, updated_at, deleted_at
        "#,
        Uuid::new_v4(),
        &req.email,
        password_hash,
        &req.full_name,
        req.phone.as_deref(),
        false
    )
    .fetch_one(db)
    .await?;
    Ok(new_user)
}

pub(super) async fn create_audit_log(
    db: &PgPool,
    user_id: Uuid,
    action: &str,
    ip_address: Option<String>,
) -> Result<(), Error> {
    query!(
        r#"
        INSERT INTO audit_logs (id, user_id, action, resource_type, resource_id, ip_address)
        VALUES ($1, $2, $3, 'user', $4, $5)
        "#,
        Uuid::new_v4(),
        user_id,
        action,
        user_id,
        ip_address.as_deref()
    )
    .execute(db)
    .await?;
    Ok(())
}
