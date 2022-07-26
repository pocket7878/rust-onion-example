use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
pub enum ApiErrorType {
    Unknown,
    UnprocessableEntity,
    RecordNotFound,
}

#[derive(Debug)]
pub struct ApiError {
    pub error_type: ApiErrorType,
    pub status: StatusCode,
    pub details: Vec<String>,
}

impl Default for ApiError {
    fn default() -> Self {
        ApiError {
            error_type: ApiErrorType::Unknown,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            details: vec![],
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "api_error": {
                "type": self.error_type,
                "status": self.status.to_string(),
                "details": self.details,
            }
        }));

        (self.status, body).into_response()
    }
}
