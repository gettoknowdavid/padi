use crate::app::AppState;
use crate::features::auth::handlers;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/verify-email", post(handlers::verify_email))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/forgot-password", post(handlers::forgot_password))
        .route("/reset-password", post(handlers::reset_password))
        .route("/phone/send-otp", post(handlers::send_otp))
        .route("/phone/verify-otp", post(handlers::verify_otp))
        .route("/google", get(handlers::google_authorize))
        .route("/google/callback", get(handlers::google_callback))
}
