use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;


use crate::{
    api_error::{ApiError, ApiErrorType},
    handler::task_detail_response::TaskDetailResponse,
};

#[debug_handler]
pub async fn postpone_task(
    Path(task_id): Path<i64>,
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case = use_case::task::PostponeTaskUseCase::new(Box::new(
        infra_provider.provide_task_repository(),
    ));
    let res = use_case.execute(task_id);
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
            } else if err.is::<use_case::task::error::TaskMaxPostponeExceededError>() {
                Err(ApiError {
                    error_type: ApiErrorType::UnprocessableEntity,
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                    details: vec![err.to_string()],
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
