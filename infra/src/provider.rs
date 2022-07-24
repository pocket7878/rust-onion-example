

pub struct Provider;

impl Provider {
    pub fn new() -> Self {
        Self
    }

    pub fn provide_task_repository(&self) -> Box<dyn domain::repository::TaskRepository> {
        // TODO: Create task repository.
        Box::new(super::repository::TaskRdbRepository::new())
    }
}