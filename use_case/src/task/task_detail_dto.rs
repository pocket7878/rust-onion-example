pub struct TaskDetailDto {
    pub id: i64,
    pub name: String,
    pub due_date: time::OffsetDateTime,
    pub postpone_count: i32,
}

impl From<domain::model::task::Task> for TaskDetailDto {
    fn from(task: domain::model::task::Task) -> Self {
        TaskDetailDto {
            id: task.id.unwrap(),
            name: task.name.value,
            due_date: task.due_date,
            postpone_count: task.postpone_count,
        }
    }
}
