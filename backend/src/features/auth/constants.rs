pub const EMAIL_VERIFY_TTL_SECS: u64 = 86_400; // 24 hours
pub const PWD_RESET_TTL_SECS: u64 = 3_600; // 1 hour
pub const OTP_TTL_SECS: u64 = 600; // 1 hour
pub const REFRESH_TOKEN_TTL_SECS: u64 = 2_592_000; // 30 days
pub const ACCESS_TOKEN_TTL_SECS: u64 = 900; // 15 minutes
pub const RATE_LIMIT_WINDOW_SECONDS: usize = 3_600;

pub const MAX_LOGIN_ATTEMPTS: i32 = 10;
pub const ACCOUNT_LOCK_HOURS: i64 = 1;
pub const OTP_RATE_LIMIT_MAX: i64 = 3;

pub const REDIS_EMAIL_VERIFY_PREFIX: &str = "email_verify:";
pub const REDIS_REFRESH_TOKEN_PREFIX: &str = "refresh_token:";
pub const REDIS_PWD_RESET_PREFIX: &str = "pwd_reset:";
pub const REDIS_OTP_PREFIX: &str = "otp:";
pub const REDIS_OTP_RATE_LIMIT_PREFIX: &str = "otp_rate:";
pub const REDIS_OTP_ATTEMPT_PREFIX: &str = "otp_attempts:";
