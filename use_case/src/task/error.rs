use domain::model::task::TaskId;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct TaskNotFoundError {
    pub task_id: TaskId,
}

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "task not found: {}", self.task_id)
    }
}

#[derive(Debug, Clone, Error)]
pub struct TaskMaxPostponeExceededError {
    pub max_postpone: i32,
}

impl fmt::Display for TaskMaxPostponeExceededError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "task can not be postponed more than {} times",
            self.max_postpone
        )
    }
}
