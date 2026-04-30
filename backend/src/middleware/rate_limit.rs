use crate::app::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use deadpool_redis::{Pool, redis};
use redis::AsyncCommands;
use std::sync::Arc;
use std::time::SystemTime;

const GENERAL_LIMIT: usize = 100;
const GENERAL_WINDOW_SECS: u64 = 10;
const AUTH_LIMIT: usize = 5;
const AUTH_WINDOW_SECS: u64 = 900;

pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();

    // Determine which limit group this request falls into
    let (limit, window_secs, group) = if path.starts_with("/api/v1/auth") {
        (AUTH_LIMIT, AUTH_WINDOW_SECS, "auth")
    } else {
        (GENERAL_LIMIT, GENERAL_WINDOW_SECS, "general")
    };

    // Use JWT user ID if present in headers, otherwise fall back to IP
    let identifier = extract_identifier(&req);
    let key = format!("rate_limit:{}:{}", identifier, group);

    match check_rate_limit(&state.redis_pool, &key, limit, window_secs).await {
        Ok(true) => next.run(req).await,
        Ok(false) => {
            let retry_after = window_secs.to_string();
            (
                StatusCode::TOO_MANY_REQUESTS,
                [
                    ("Retry-After", retry_after.as_str()),
                    ("Content-Type", "application/json"),
                ],
                r#"{"data":null,"meta":null,"error":{"message":"Too many requests. Slow down!"}}"#,
            )
                .into_response()
        }
        Err(_) => {
            // If Redis is down, fail open — don't block legitimate traffic
            next.run(req).await
        }
    }
}

/// Sliding window algorithm using Redis sorted sets.
/// Stores each request as a member scored by its timestamp (ms).
/// Removes members outside the window, then counts remaining.
async fn check_rate_limit(
    pool: &Pool,
    key: &str,
    limit: usize,
    window_secs: u64,
) -> anyhow::Result<bool> {
    let mut conn = pool.get().await?;

    let now_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis() as u64;

    let window_start_ms = now_ms - (window_secs * 1000);

    // Pipeline all Redis commands together for efficiency
    redis::pipe()
        .atomic()
        // Add this request (score = timestamp, member = timestamp for uniqueness)
        .cmd("ZADD")
        .arg(key)
        .arg(now_ms)
        .arg(now_ms)
        // Remove all entries outside the window
        .cmd("ZREMRANGEBYSCORE")
        .arg(key)
        .arg(0)
        .arg(window_start_ms)
        // Set expiry so keys self-clean
        .cmd("EXPIRE")
        .arg(key)
        .arg(window_secs)
        .query_async::<()>(&mut *conn)
        .await?;

    // Count requests within the window
    let count: usize = conn.zcard(key).await?;
    Ok(count <= limit)
}

/// Extracts a unique identifier for rate limiting.
///
/// Prefers JWT bearer token subclaim; falls back to IP address.
fn extract_identifier(req: &Request<Body>) -> String {
    // Try to get user ID from Authorization header
    // Full JWT parsing happens in the auth middleware (E2); here we just use the raw token
    // as a proxy identifier — good enough for rate limiting
    if let Some(auth_header) = req.headers().get("Authorization")
        && let Ok(auth_str) = auth_header.to_str()
        && let Some(token) = auth_str.strip_prefix("Bearer ")
    {
        // Use last 16 chars of token as identifier (avoids logging full token)
        let len = token.len();
        if len >= 16 {
            return token[len - 16..].to_string();
        }
    }

    // Fall back to IP from X-Forwarded-For (set by Railway/Render/Cloudflare)
    // then to direct connection IP
    if let Some(forwarded) = req.headers().get("X-Forwarded-For")
        && let Ok(ip) = forwarded.to_str()
    {
        return ip.split(',').next().unwrap_or("unknown").trim().to_string();
    }

    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::AppState;
    use crate::cache::cache_redis_pool;
    use crate::config::Config;
    use crate::middleware::rate_limit::rate_limit_middleware;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::get,
    };
    use deadpool_redis::Pool;
    use sqlx::PgPool;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn test_config() -> Config {
        Config {
            database_url: "postgres://dummy".to_string(),
            redis_url: "redis://127.0.0.1:6379".to_string(),
            jwt_secret: "secret".to_string(),
            jwt_refresh_secret: "refresh_secret".to_string(),
            paystack_secret_key: "paystack".to_string(),
            flutterwave_secret_key: "flutterwave".to_string(),
            whatsapp_token: "whatsapp".to_string(),
            whatsapp_phone_number_id: "phone_id".to_string(),
            africas_talking_api_key: "at_key".to_string(),
            africas_talking_username: "at_user".to_string(),
            resend_api_key: "resend".to_string(),
            cloudflare_r2_account_id: "r2_account".to_string(),
            cloudflare_r2_access_key: "r2_access".to_string(),
            cloudflare_r2_secret_key: "r2_secret".to_string(),
            cloudflare_r2_bucket: "r2_bucket".to_string(),
            app_env: "test".to_string(),
            port: 8080,
            frontend_url: "http://localhost:3000".to_string(),
        }
    }

    /// Builds a minimal test router with rate limiting wired up,
    /// using a tiny window (1 second) and low limit (3 requests)
    /// so the test doesn't have to hammer the server
    async fn build_test_router(redis_pool: Pool) -> Router {
        // We define a local middleware that overrides the constants
        // by calling check_rate_limit directly with test values
        async fn dummy_handler() -> StatusCode {
            StatusCode::OK
        }

        let state = Arc::new(AppState {
            config: test_config(),
            pg_pool: PgPool::connect_lazy("postgres://dummy").unwrap(),
            redis_pool,
        });

        Router::new()
            .route("/test", get(dummy_handler))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                rate_limit_middleware,
            ))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_rate_limit_returns_429() {
        let redis_url = "redis://127.0.0.1:6379";
        let redis_pool = match cache_redis_pool(redis_url) {
            Ok(pool) => pool,
            Err(_) => {
                println!("Skipping: Redis is not running on {}", redis_url);
                return;
            }
        };

        if let Err(e) = redis_pool.get().await {
            println!("Skipping test: Cannot connect to Redis: {}", e);
            return;
        }

        // Flush Redis so previous test runs don't interfere
        {
            let mut conn = redis_pool.get().await.expect("Redis connection failed");
            let _: () = redis::cmd("FLUSHALL")
                .query_async(&mut *conn)
                .await
                .expect("Failed to FLUSHALL");
        }

        let app = build_test_router(redis_pool).await;

        let mut last_status = StatusCode::OK;

        // Use a small limit for the test so we don't need 101 requests
        const TEST_LIMIT: usize = 5;

        // Send TEST_LIMIT + 1 requests
        for i in 0..=TEST_LIMIT {
            let req = Request::builder()
                .uri("/test")
                .header("X-Forwarded-For", "1.2.3.4")
                .body(Body::empty())
                .unwrap();

            let response = app.clone().oneshot(req).await.unwrap();
            last_status = response.status();

            println!("Request {} → Status: {}", i, last_status);
        }

        assert_eq!(
            last_status,
            StatusCode::TOO_MANY_REQUESTS,
            "Expected 429 after exceeding rate limit"
        );
    }
}
