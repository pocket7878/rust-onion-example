use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::api_error::{ApiError, ApiErrorType};

#[derive(Serialize, Debug)]
pub struct FetchTaskResponse {
    id: i64,
    name: String,
    #[serde(with = "time::serde::rfc3339")]
    due_date: time::OffsetDateTime,
    postpone_count: i32,
}

#[debug_handler]
pub async fn fetch_task(
    Path(task_id): Path<i64>,
    Extension(infra_provider): Extension<Arc<infra::Provider>>,
) -> Result<Json<FetchTaskResponse>, ApiError> {
    let use_case =
        use_case::task::FetchTaskUseCase::new(Box::new(infra_provider.provide_task_repository()));
    match use_case.execute(&task_id).await {
        Ok(task) => match task {
            Some(task) => Ok(Json(FetchTaskResponse {
                id: task.id.unwrap(),
                name: task.name.value,
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
