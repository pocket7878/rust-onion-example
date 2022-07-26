use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

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
    Json(payload): Json<UpdateTaskParams>,
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case =
        use_case::task::UpdateTaskUseCase::new(Box::new(infra_provider.provide_task_repository()));
    let res = use_case.execute(task_id, &payload.name);
    match res.await {
        Ok(task) => Ok(Json(TaskDetailResponse {
            id: task.id.unwrap(),
            name: task.name.value,
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
