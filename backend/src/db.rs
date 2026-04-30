use std::time::Duration;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(100))
        .connect(database_url)
        .await
        .expect("Failed to connect to the PostgreSQL")
}
