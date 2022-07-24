use std::error::Error;
use domain::{repository::TaskRepository, model::{task::{TaskId, Task, TaskName}}};

pub struct CreateTaskUseCase {
		task_repository: Box<dyn TaskRepository>,
}

impl CreateTaskUseCase {
		pub fn new(task_repository: Box<dyn TaskRepository>) -> Self {
				Self {
						task_repository,
				}
		}

		pub async fn execute(&self, task_name: &str, due_date: time::Date, user_id: i32) -> Result<TaskId, Box<dyn Error>> {
			let mut task = Task::new(TaskName::new(task_name), user_id, due_date);
			self.task_repository.insert(&mut task).await?;

			task.get_id().ok_or("TaskId should be filled after insert".into())
		}
}