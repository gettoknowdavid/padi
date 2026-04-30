use axum::{Router, http::StatusCode, middleware, routing::get};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::config::Config;
use crate::middleware::rate_limit::rate_limit_middleware;
use crate::routes::health::health_handler;

/// Shared application state. Wrapping this in an [Arc] so it can cheaply be cloned across
/// handler threads
pub struct AppState {
    pub config: Config,
    pub pg_pool: PgPool,
    pub redis_pool: RedisPool,
}

/// Build the application router
pub async fn build_router(config: Config, pg_pool: PgPool, redis_pool: RedisPool) -> Router {
    let state = Arc::new(AppState {
        config,
        pg_pool,
        redis_pool,
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
        .route("/health", get(health_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .layer(middleware_stack)
        .with_state(state)
}
