use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::api_error::{ApiError, ApiErrorType};

#[derive(Deserialize, Debug)]
pub struct CreateTaskParams {
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    due_date: time::OffsetDateTime,
}

#[derive(Serialize, Debug)]
pub struct CreateTaskResponse {
    task_id: i64,
}

#[debug_handler]
pub async fn create_task(
    Json(payload): Json<CreateTaskParams>,
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<CreateTaskResponse>, ApiError> {
    let use_case =
        use_case::task::CreateTaskUseCase::new(Box::new(infra_provider.provide_task_repository()));
    let task_id = use_case.execute(&payload.name, payload.due_date);
    match task_id.await {
        Ok(task_id) => Ok(Json(CreateTaskResponse { task_id })),
        Err(err) => Err(ApiError {
            error_type: ApiErrorType::UnprocessableEntity,
            status: StatusCode::UNPROCESSABLE_ENTITY,
            details: vec![err.to_string()],
        }),
    }
}
