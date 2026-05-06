use crate::errors::AppError;
use deadpool_redis::{Connection, Pool};
use uuid::Uuid;

async fn redis_conn(pool: &Pool) -> Result<Connection, AppError> {
    pool.get().await.map_err(|e| {
        tracing::error!("Redis connection failed: {:?}", e);
        AppError::Internal("Redis connection failed".to_string())
    })
}

pub async fn delete_refresh_token(pool: &Pool, refresh_token: &str) -> Result<(), AppError> {
    let key = format!("refresh_token:{}", refresh_token);
    let mut conn = redis_conn(pool).await?;
    redis::cmd("DEL")
        .arg(&key)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to delete refresh token: {:?}", error);
            AppError::Internal("Failed to delete refresh token".to_string())
        })
}

pub async fn create_refresh_token(pool: &Pool, user_id: &str) -> Result<String, AppError> {
    let token = uuid::Uuid::new_v4().to_string();
    let key = format!("refresh_token:{}", token);
    let mut conn = redis_conn(pool).await?;
    redis::cmd("SET")
        .arg(&key)
        .arg(user_id)
        .arg("EX")
        .arg(2_592_000u64)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to store refresh token: {:?}", error);
            AppError::Internal("Failed to store refresh token".to_string())
        })?;
    Ok(token)
}

pub async fn rotate_refresh_token(
    pool: &Pool,
    old_token: &str,
) -> Result<(String, String), AppError> {
    let key = format!("refresh_token:{}", old_token);
    let mut conn = redis_conn(pool).await?;
    let user_id: Option<String> = redis::cmd("GET")
        .arg(&key)
        .query_async(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to get refresh token: {:?}", error);
            AppError::Internal("Failed to get refresh token".to_string())
        })?;

    match user_id {
        None => {
            // Token isn't found — possible reuse attack
            // Revoke all tokens for this user if you can identify them
            Err(AppError::Unauthorized(
                "Invalid or expired refresh token".into(),
            ))
        }
        Some(uuid) => {
            delete_refresh_token(pool, old_token).await?;

            let new_token = create_refresh_token(pool, &uuid).await?;
            Ok((uuid, new_token))
        }
    }
}

pub async fn create_pwd_reset_token(pool: &Pool, user_id: &str) -> Result<String, AppError> {
    let token = Uuid::new_v4().to_string();
    let key = format!("pwd_reset:{}", token);
    let mut conn = redis_conn(pool).await?;
    redis::cmd("SET")
        .arg(&key)
        .arg(user_id)
        .arg("EX")
        .arg(3600)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to store password reset token: {:?}", error);
            AppError::Internal("Failed to store password reset token".to_string())
        })?;
    Ok(token)
}

pub async fn get_pwd_reset_token(pool: &Pool, token: &str) -> Result<Option<String>, AppError> {
    let key = format!("pwd_reset:{}", token);
    let mut conn = redis_conn(pool).await?;
    redis::cmd("GET")
        .arg(&key)
        .query_async(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to get User ID Password Reset Token: {:?}", error);
            AppError::Internal("Failed to get User ID Password Reset Token".to_string())
        })
}

pub async fn delete_pwd_reset_token(pool: &Pool, token: &str) -> Result<(), AppError> {
    let key = format!("pwd_reset:{}", token);
    let mut conn = redis_conn(pool).await?;
    redis::cmd("DEL")
        .arg(&key)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|error| {
            tracing::error!("Failed to delete Password Reset Token: {:?}", error);
            AppError::Internal("Failed to delete Password Reset Token".to_string())
        })
}
