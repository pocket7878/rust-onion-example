use use_case::task::CreateTaskUseCase;

pub struct DiContainer {
    provider: infra::Provider,
}

impl DiContainer {
    pub fn new(provider: infra::Provider) -> Self {
        Self { provider }
    }

    pub fn create_task_use_case(&self) -> CreateTaskUseCase {
        use_case::task::CreateTaskUseCase::new(Box::new(self.provider.provide_task_repository()))
    }

    pub fn fetch_task_use_case(&self) -> use_case::task::FetchTaskUseCase {
        use_case::task::FetchTaskUseCase::new(Box::new(self.provider.provide_task_repository()))
    }

    pub fn list_tasks_use_case(&self) -> use_case::task::ListTasksUseCase {
        use_case::task::ListTasksUseCase::new(Box::new(self.provider.provide_task_repository()))
    }

    pub fn postpone_task_use_case(&self) -> use_case::task::PostponeTaskUseCase {
        use_case::task::PostponeTaskUseCase::new(Box::new(self.provider.provide_task_repository()))
    }

    pub fn update_task_use_case(&self) -> use_case::task::UpdateTaskUseCase {
        use_case::task::UpdateTaskUseCase::new(Box::new(self.provider.provide_task_repository()))
    }
}
