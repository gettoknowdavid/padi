use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Response bodies ─────────────────────────────────────────
#[derive(Deserialize)]
pub struct CreateOrgRequest {
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct InviteRequest {
    pub email: String,
    pub role: String,
}

// ── Response bodies ─────────────────────────────────────────
#[derive(Deserialize, Serialize)]
pub struct OrgResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub subscription_plan: String,
}
#[derive(Serialize)]
pub struct CreateOrgResponse {
    pub org: OrgResponse,
    pub access_token: String,
}
