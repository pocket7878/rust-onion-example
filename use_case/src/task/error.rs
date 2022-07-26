use std::fmt;

use domain::model::task::TaskId;

#[derive(Debug, Clone)]
pub struct TaskNotFoundError {
    pub task_id: TaskId,
}

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "task not found: {}", self.task_id)
    }
}
impl std::error::Error for TaskNotFoundError {}
