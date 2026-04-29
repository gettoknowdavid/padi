use anyhow::{Context, Result};
use std::env::var;
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub paystack_secret_key: String,
    pub flutterwave_secret_key: String,
    pub whatsapp_token: String,
    pub whatsapp_phone_number_id: String,
    pub africas_talking_api_key: String,
    pub africas_talking_username: String,
    pub resend_api_key: String,
    pub cloudflare_r2_account_id: String,
    pub cloudflare_r2_access_key: String,
    pub cloudflare_r2_secret_key: String,
    pub cloudflare_r2_bucket: String,
    pub app_env: String,
    pub port: u16,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        let config: Config = Config {
            database_url: var("DATABASE_URL").context("DATABASE_URL is missing")?,
            redis_url: var("REDIS_URL").context("REDIS_URL is missing")?,
            jwt_secret: var("JWT_SECRET").context("JWT_SECRET is missing")?,
            jwt_refresh_secret: var("JWT_REFRESH_SECRET")
                .context("JWT_REFRESH_SECRET is missing")?,
            paystack_secret_key: var("PAYSTACK_SECRET_KEY")
                .context("PAYSTACK_SECRET_KEY is missing")?,
            flutterwave_secret_key: var("FLUTTERWAVE_SECRET_KEY")
                .context("FLUTTERWAVE_SECRET_KEY is missing")?,
            whatsapp_token: var("WHATSAPP_TOKEN").context("WHATSAPP_TOKEN is missing")?,
            whatsapp_phone_number_id: var("WHATSAPP_PHONE_NUMBER_ID")
                .context("WHATSAPP_PHONE_NUMBER_ID is missing")?,
            africas_talking_api_key: var("AFRICAS_TALKING_API_KEY")
                .context("AFRICAS_TALKING_API_KEY is missing")?,
            africas_talking_username: var("AFRICAS_TALKING_USERNAME")
                .context("AFRICAS_TALKING_USERNAME is missing")?,
            resend_api_key: var("RESEND_API_KEY").context("RESEND_API_KEY is missing")?,
            cloudflare_r2_account_id: var("CLOUDFLARE_R2_ACCOUNT_ID")
                .context("CLOUDFLARE_R2_ACCOUNT_ID is missing")?,
            cloudflare_r2_access_key: var("CLOUDFLARE_R2_ACCESS_KEY")
                .context("CLOUDFLARE_R2_ACCESS_KEY is missing")?,
            cloudflare_r2_secret_key: var("CLOUDFLARE_R2_SECRET_KEY")
                .context("CLOUDFLARE_R2_SECRET_KEY is missing")?,
            cloudflare_r2_bucket: var("CLOUDFLARE_R2_BUCKET")
                .context("CLOUDFLARE_R2_BUCKET is missing")?,
            app_env: var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            port: var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .context("PORT must be a valid number")?,
            frontend_url: var("FRONTEND_URL").context("FRONTEND_URL is missing")?,
        };

        Ok(config)
    }
}
