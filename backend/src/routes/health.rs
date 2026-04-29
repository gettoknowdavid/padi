use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;

// Handler returns: { "data": { "status": "ok", "version": "0.1.0" }, "meta": null, "error": null }
pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "data": {
                "status": "ok",
                "version": "0.1.0",
            },
            "meta": null,
            "error": null
        })),
    )
}
