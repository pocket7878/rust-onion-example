use time::OffsetDateTime;

pub type TaskId = i64;

#[derive(Debug)]
pub struct Task {
    pub id: Option<TaskId>,
    pub name: TaskName,
    pub postpone_count: i32,
    pub due_date: OffsetDateTime,
}

impl Task {
    pub fn new(name: TaskName, due_date: OffsetDateTime) -> Self {
        // TODO: Validate domain logic.
        Task {
            id: None,
            name,
            postpone_count: 0,
            due_date,
        }
    }

    pub fn reconstruct(
        id: TaskId,
        name: TaskName,
        postpone_count: i32,
        due_date: OffsetDateTime,
    ) -> Self {
        Task {
            id: Some(id),
            name,
            postpone_count,
            due_date,
        }
    }
}

#[derive(Debug)]
pub struct TaskName {
    pub value: String,
}

impl TaskName {
    pub fn new(name: &str) -> Self {
        // TODO: Validate domain logic.
        Self {
            value: name.to_string(),
        }
    }

    pub fn reconstruct(name: &str) -> Self {
        Self {
            value: name.to_string(),
        }
    }
}
