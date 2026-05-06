use crate::app::AppState;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use std::sync::Arc;

pub mod auth;
pub mod health;

use crate::features::auth::middleware::auth_middleware;
use health::health_handler;

pub fn build_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    // Protected routes — JWT required
    let protected_routes = Router::new()
        // .route("/health", get(health_handler))
        .layer(from_fn_with_state(state, auth_middleware));

    // Public routes
    let public_routes = Router::new()
        .route("/health", get(health_handler))
        .nest("/api/v1/auth", auth::router());

    Router::new().merge(public_routes).merge(protected_routes)
}
