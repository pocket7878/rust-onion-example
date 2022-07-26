use domain::{
    model::task::{Task, TaskId, TaskName},
    repository::TaskRepository,
};
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct TaskNotFoundError {
    task_id: TaskId,
}

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "task not found: {}", self.task_id)
    }
}
impl std::error::Error for TaskNotFoundError {}

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
    ) -> Result<Task, Box<dyn Error + Send + Sync + 'static>> {
        let task = self.task_repository.find_by_id(&id).await?;
        if let Some(mut task) = task {
            task.name = TaskName::new(name);
            self.task_repository.update(&task).await?;

            Ok(task)
        } else {
            return Err(TaskNotFoundError { task_id: id }.into());
        }
    }
}
