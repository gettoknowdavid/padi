use crate::config::Config;
use crate::db::create_pool;
use crate::routes::health::health_handler;
use axum::{Router, http::StatusCode, routing::get};
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
    pub pg_pool: PgPool,
}

/// Build the application router
pub async fn build_router(config: Config) -> Router {
    let pg_pool = create_pool(config.database_url.as_str()).await;
    let state = Arc::new(AppState { config, pg_pool });

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
        .layer(middleware_stack)
        .with_state(state)
}
