use domain::{model::task::Task, repository::TaskRepository};
use std::error::Error;

pub struct ListTasksUseCase {
    task_repository: Box<dyn TaskRepository + Send + Sync>,
}

impl ListTasksUseCase {
    pub fn new(task_repository: Box<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Task>, Box<dyn Error + Send + Sync + 'static>> {
        let tasks = self.task_repository.list().await?;
        Ok(tasks)
    }
}
