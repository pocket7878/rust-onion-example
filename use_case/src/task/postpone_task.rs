use super::TaskDetailDto;
use domain::{model::task::TaskId, repository::TaskRepository};

pub struct PostponeTaskUseCase {
    task_repository: Box<dyn TaskRepository + Send + Sync>,
}

impl PostponeTaskUseCase {
    pub fn new(task_repository: Box<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(
        &self,
        id: TaskId,
    ) -> Result<TaskDetailDto, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let task = self.task_repository.find_by_id(&id).await?;
        if let Some(mut task) = task {
            match task.postpone() {
                Ok(()) => {
                    self.task_repository.update(&task).await?;
                    Ok(task.into())
                }
                Err(e) => {
                    if let Some(e) =
                        e.downcast_ref::<domain::model::task::MaxPostponeCountExceededError>()
                    {
                        Err(super::error::TaskMaxPostponeExceededError {
                            max_postpone: e.max_postpone,
                        }
                        .into())
                    } else {
                        Err(e.into())
                    }
                }
            }
        } else {
            return Err(super::error::TaskNotFoundError { task_id: id }.into());
        }
    }
}
