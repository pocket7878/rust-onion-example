use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::{
    api_error::{ApiError, ApiErrorType},
    handler::task_detail_response::TaskDetailResponse,
};

#[derive(Deserialize, Debug)]
pub struct UpdateTaskParams {
    name: String,
}

#[debug_handler]
pub async fn update_task(
    Path(task_id): Path<i64>,
    Extension(di_container): Extension<Arc<di::DiContainer>>,
    Json(payload): Json<UpdateTaskParams>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case = di_container.update_task_use_case();
    let res = use_case.execute(task_id, &payload.name);
    match res.await {
        Ok(task) => Ok(Json(TaskDetailResponse {
            id: task.id,
            name: task.name,
            due_date: task.due_date,
            postpone_count: task.postpone_count,
        })),
        Err(err) => {
            if err.is::<use_case::task::error::TaskNotFoundError>() {
                Err(ApiError {
                    error_type: ApiErrorType::RecordNotFound,
                    status: StatusCode::NOT_FOUND,
                    ..Default::default()
                })
            } else {
                Err(ApiError {
                    error_type: ApiErrorType::UnprocessableEntity,
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    details: vec![err.to_string()],
                })
            }
        }
    }
}
