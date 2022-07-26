use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TaskDetailResponse {
    pub id: i64,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub due_date: time::OffsetDateTime,
    pub postpone_count: i32,
}
