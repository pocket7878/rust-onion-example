use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;


use crate::{
    api_error::{ApiError, ApiErrorType},
    handler::task_detail_response::TaskDetailResponse,
};

#[debug_handler]
pub async fn list_tasks(
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<Vec<TaskDetailResponse>>, ApiError> {
    let use_case =
        use_case::task::ListTasksUseCase::new(Box::new(infra_provider.provide_task_repository()));
    match use_case.execute().await {
        Ok(tasks) => {
            let task_details = tasks
                .into_iter()
                .map(|task| TaskDetailResponse {
                    id: task.id.unwrap(),
                    name: task.name.value,
                    due_date: task.due_date,
                    postpone_count: task.postpone_count,
                })
                .collect();
            Ok(Json(task_details))
        }
        Err(err) => Err(ApiError {
            error_type: ApiErrorType::Unknown,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            details: vec![err.to_string()],
        }),
    }
}
