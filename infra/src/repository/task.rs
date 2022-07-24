use async_trait::async_trait;
use domain::{repository, model::task::{Task, TaskId}};

pub struct TaskRdbRepository {}

impl TaskRdbRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl repository::TaskRepository for TaskRdbRepository {
    async fn find_by_id(&self, _id: &TaskId) -> Result<domain::model::task::Task, Box<dyn std::error::Error>> {
        // TODO: Fetch task from db and return as domain model. Use Task::reconstruct to fill fields.
        unimplemented!()
    }

    async fn insert(&self, _task: &mut Task) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Insert task to db. And set task.id to new id.
        unimplemented!()
    }

    async fn update(&self, _task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Update task in db.
        unimplemented!()
    }
}
