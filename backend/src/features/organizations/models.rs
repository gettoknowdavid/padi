use crate::features::organizations::dto::OrgResponse;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub address: Option<String>,
    pub state_code: Option<String>,
    pub lga: Option<String>,
    pub cac_number: Option<String>,
    pub tin: Option<String>,
    pub phone: Option<String>,
    pub whatsapp_number: Option<String>,
    pub email: Option<String>,
    pub subscription_plan: String,
    pub sms_sender_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
impl Organization {
    pub fn into_response(self) -> OrgResponse {
        OrgResponse {
            id: self.id,
            name: self.name,
            slug: self.slug,
            subscription_plan: self.subscription_plan,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct Invitation {
    pub id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub role: String,
    pub token: String,
    pub invited_by: Uuid,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
