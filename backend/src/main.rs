pub mod app;
mod config;
pub mod errors;
pub mod db;

pub mod routes {
    pub mod health;
}

use crate::app::build_router;
use anyhow::Result;
use config::Config;
use tokio::{net::TcpListener, signal};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load `.env` before anything reads env vars
    dotenvy::dotenv().ok();

    // Initialize structure logging
    // Format: `JSON` in production, pretty in development
    // Level: Controlled by RUST_LOG env var
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer().json())
        .init();

    // Load and validate all configs — fails fast with a clear message if any required env var
    // is missing
    let config = Config::from_env()?;
    let port = config.port;
    let app_env = config.app_env.clone();

    // Build the Axum router with all middleware
    let router = build_router(config).await;

    // Bind the TCP listener
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;

    info!(port = port, env = %app_env, "Padi API is starting");

    // Serve with graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shut down cleanly");

    Ok(())
}

/// Listens for `CTRL+C` (SIGINT) or SIGTERM (from Railway/Render/Docker) and resolves the
/// future — `axum::serve` then drains in-flight requests before exiting
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install the CRTL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    // On non-Unix (Windows dev machines), SIGTERM doesn't exist — fall back to waiting
    // forever to waiting forever so ctrl_c still works
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
