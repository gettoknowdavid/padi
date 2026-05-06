use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: &str, message: &str) -> ApiResponse<()> {
        ApiResponse {
            data: None,
            error: Some(ApiError {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }
}
