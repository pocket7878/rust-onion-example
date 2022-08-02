use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;

use crate::{
    api_error::{ApiError, ApiErrorType},
    handler::task_detail_response::TaskDetailResponse,
};

#[debug_handler]
pub async fn fetch_task(
    Path(task_id): Path<i64>,
    Extension(di_container): Extension<Arc<di::DiContainer>>,
) -> Result<Json<TaskDetailResponse>, ApiError> {
    let use_case = di_container.fetch_task_use_case();
    match use_case.execute(task_id).await {
        Ok(task) => match task {
            Some(task) => Ok(Json(TaskDetailResponse {
                id: task.id,
                name: task.name,
                due_date: task.due_date,
                postpone_count: task.postpone_count,
            })),
            None => Err(ApiError {
                error_type: ApiErrorType::RecordNotFound,
                status: StatusCode::NOT_FOUND,
                ..Default::default()
            }),
        },
        Err(err) => Err(ApiError {
            error_type: ApiErrorType::Unknown,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            details: vec![err.to_string()],
        }),
    }
}
