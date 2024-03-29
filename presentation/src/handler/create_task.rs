use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

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
    Extension(di_container): Extension<Arc<di::DiContainer>>,
    Json(payload): Json<CreateTaskParams>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case = di_container.create_task_use_case();
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
