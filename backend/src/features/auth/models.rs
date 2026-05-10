#![allow(dead_code)]

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>, 
    pub password_hash: Option<String>,
    pub is_verified: bool,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn into_response(self) -> crate::features::auth::dto::UserResponse {
        crate::features::auth::dto::UserResponse {
            id: self.id,
            email: self.email,
            phone: self.phone,
            full_name: self.full_name,
            is_verified: self.is_verified,
        }
    }
}
