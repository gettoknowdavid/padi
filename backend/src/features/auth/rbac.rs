use crate::errors::AppError;
use crate::features::auth::middleware::AuthUser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Agent = 0,
    Support = 1,
    Sales = 2,
    Admin = 3,
    Owner = 4,
}

impl Role {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "owner" => Some(Role::Owner),
            "admin" => Some(Role::Admin),
            "sales" => Some(Role::Sales),
            "support" => Some(Role::Support),
            "agent" => Some(Role::Agent),
            _ => None,
        }
    }
}

pub fn require_at_least(auth: &AuthUser, min: Role) -> Result<(), AppError> {
    let user_role = auth
        .role
        .as_deref()
        .and_then(Role::from_str)
        .ok_or(AppError::Forbidden)?;

    if user_role >= min {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
