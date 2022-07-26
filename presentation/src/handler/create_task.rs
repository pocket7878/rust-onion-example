use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    api_error::{ApiError, ApiErrorType},
    handler::task_detail_response::TaskDetailResponse,
};

#[derive(Deserialize, Debug)]
pub struct CreateTaskParams {
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    due_date: time::OffsetDateTime,
}

#[debug_handler]
pub async fn create_task(
    Json(payload): Json<CreateTaskParams>,
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case =
        use_case::task::CreateTaskUseCase::new(Box::new(infra_provider.provide_task_repository()));
    let res = use_case.execute(&payload.name, payload.due_date);
    match res.await {
        Ok(task) => Ok(Json(TaskDetailResponse {
            id: task.id,
            name: task.name,
            due_date: task.due_date,
            postpone_count: task.postpone_count,
        })),
        Err(err) => Err(ApiError {
            error_type: ApiErrorType::UnprocessableEntity,
            status: StatusCode::UNPROCESSABLE_ENTITY,
            details: vec![err.to_string()],
        }),
    }
}
