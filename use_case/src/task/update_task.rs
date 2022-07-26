use domain::{
    model::task::{Task, TaskId, TaskName},
    repository::TaskRepository,
};
use std::{error::Error, fmt};

use super::TaskDetailDto;

pub struct UpdateTaskUseCase {
    task_repository: Box<dyn TaskRepository + Send + Sync>,
}

impl UpdateTaskUseCase {
    pub fn new(task_repository: Box<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(
        &self,
        id: TaskId,
        name: &str,
    ) -> Result<TaskDetailDto, Box<dyn Error + Send + Sync + 'static>> {
        let task = self.task_repository.find_by_id(&id).await?;
        if let Some(mut task) = task {
            task.name = TaskName::new(name);
            self.task_repository.update(&task).await?;

            Ok(task.into())
        } else {
            return Err(super::error::TaskNotFoundError { task_id: id }.into());
        }
    }
}
