use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org_id: Option<String>,
    pub role: Option<String>,
    pub iat: u64,
    pub exp: u64,
}

pub struct CreateTokenArgs {
    pub user_id: String,
    pub org_id: Option<String>,
    pub role: Option<String>,
    pub secret: String,
}

pub fn create_access_token(args: CreateTokenArgs) -> Result<String, Error> {
    let now = chrono::Utc::now().timestamp() as u64;
    let claims = Claims {
        sub: args.user_id,
        org_id: args.org_id,
        role: args.role,
        iat: now,
        exp: now + 900, // 15 minutes
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(args.secret.as_bytes()),
    )
}

pub fn decode_access_token(token: &str, secret: &str) -> Result<Claims, Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
