pub const EMAIL_VERIFY_TTL_SECS: u64 = 86_400;     // 24 hours
pub const PWD_RESET_TTL_SECS: u64 = 3_600;          // 1 hour
pub const REFRESH_TOKEN_TTL_SECS: u64 = 2_592_000;  // 30 days
pub const ACCESS_TOKEN_TTL_SECS: u64 = 900;          // 15 minutes
pub const MAX_LOGIN_ATTEMPTS: i32 = 10;
pub const ACCOUNT_LOCK_HOURS: i64 = 1;

pub const REDIS_EMAIL_VERIFY_PREFIX: &str = "email_verify:";
pub const REDIS_REFRESH_TOKEN_PREFIX: &str = "refresh_token:";
pub const REDIS_PWD_RESET_PREFIX: &str = "pwd_reset:";