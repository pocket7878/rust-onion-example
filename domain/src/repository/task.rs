extern crate async_trait;

use async_trait::async_trait;
use std::error::Error;

use crate::model::task::{TaskId, Task};

#[async_trait]
pub trait TaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Task, Box<dyn Error>>;
    async fn insert(&self, task: &mut Task) -> Result<(), Box<dyn Error>>;
    async fn update(&self, task: &Task) -> Result<(), Box<dyn Error>>;
}
