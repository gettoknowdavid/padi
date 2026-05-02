use crate::app::AppState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub mod auth;
pub mod health;

use health::health_handler;

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_handler))
        .nest("/api/v1/auth", auth::router())
}
