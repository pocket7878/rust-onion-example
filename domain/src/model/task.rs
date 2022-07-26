use thiserror::Error;

use time::{Duration, OffsetDateTime};

const MAX_POSTPONE: i32 = 3;

pub type TaskId = i64;

#[derive(Debug)]
pub struct Task {
    pub id: Option<TaskId>,
    pub name: TaskName,
    pub postpone_count: i32,
    pub due_date: OffsetDateTime,
}

#[derive(Debug, Clone, Error)]
pub struct MaxPostponeCountExceededError {
    pub max_postpone: i32,
}

impl std::fmt::Display for MaxPostponeCountExceededError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "task can not be postponed more than {} times",
            MAX_POSTPONE
        )
    }
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

    pub fn postpone(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use std::ops::Add;

        if self.postpone_count >= MAX_POSTPONE {
            Err(MaxPostponeCountExceededError {
                max_postpone: MAX_POSTPONE,
            }
            .into())
        } else {
            self.postpone_count += 1;
            self.due_date = self.due_date.add(Duration::days(7));
            Ok(())
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
