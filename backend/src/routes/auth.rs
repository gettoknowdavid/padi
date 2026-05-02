use crate::app::AppState;
use crate::auth::handlers;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(handlers::register))
}
