use domain::{
    model::task::{Task, TaskId},
    repository::TaskRepository,
};
use std::error::Error;

pub struct FetchTaskUseCase {
    task_repository: Box<dyn TaskRepository + Send + Sync>,
}

impl FetchTaskUseCase {
    pub fn new(task_repository: Box<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(
        &self,
        id: &TaskId,
    ) -> Result<Option<Task>, Box<dyn Error + Send + Sync + 'static>> {
        let task = self.task_repository.find_by_id(id).await?;

        Ok(task)
    }
}
