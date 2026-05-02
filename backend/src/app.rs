use crate::config::Config;
use crate::middleware::rate_limit::rate_limit_middleware;
use axum::{Router, http::StatusCode, middleware};
use deadpool_redis::Pool as RedisPool;
use reqwest::Client;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

/// Shared application state. Wrapping this in an [Arc] so it can cheaply be cloned across
/// handler threads
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub redis: RedisPool,
    pub http_client: Client,
}

/// Build the application router
pub async fn build_router(config: Config, pg_pool: PgPool, redis_pool: RedisPool) -> Router {
    let state = Arc::new(AppState {
        config,
        db: pg_pool,
        redis: redis_pool,
        http_client: Client::new(),
    });

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );

    Router::new()
        .merge(crate::routes::build_routes())
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(middleware_stack)
        .with_state(state)
}
