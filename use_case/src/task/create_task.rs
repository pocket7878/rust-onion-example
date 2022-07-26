use domain::{
    model::task::{Task, TaskId, TaskName},
    repository::TaskRepository,
};
use std::error::Error;

pub struct CreateTaskUseCase {
    task_repository: Box<dyn TaskRepository + Send + Sync>,
}

impl CreateTaskUseCase {
    pub fn new(task_repository: Box<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(
        &self,
        task_name: &str,
        due_date: time::OffsetDateTime,
    ) -> Result<Task, Box<dyn Error + Send + Sync + 'static>> {
        tracing::debug!(
            "CreateTaskUseCase::execute name = {}, due_date = {}",
            task_name,
            due_date
        );
        let mut task = Task::new(TaskName::new(task_name), due_date);
        self.task_repository.insert(&mut task).await?;

        Ok(task)
    }
}
