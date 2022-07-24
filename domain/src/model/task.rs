use time::{Date};
use super::user::UserId;

pub type TaskId = i32;

#[derive(Debug)]
pub struct Task {
    id: Option<TaskId>,
    name: TaskName,
    user_id: UserId,
    postpone_count: i32,
    due_date: Date,
}

impl Task {
    pub fn new(name: TaskName, user_id: UserId, due_date: Date) -> Self {
        // TODO: Validate domain logic.
        Task {
            id: None,
            name,
            user_id,
            postpone_count: 0,
            due_date: due_date,
        }
    }

    pub fn reconstruct(id: TaskId, name: TaskName, user_id: UserId, postpone_count: i32, due_date: Date) -> Self {
        Task {
            id: Some(id),
            name,
            user_id,
            postpone_count,
            due_date,
        }
    }

    pub fn get_id(&self) -> Option<TaskId> {
        self.id
    }
}

#[derive(Debug)]
pub struct TaskName {
    value: String,
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