extern crate async_trait;

use async_trait::async_trait;
use std::error::Error;

use crate::model::user::{UserId, User};

#[async_trait]
pub trait UserRepository {
    async fn findById(&self, id: UserId) -> Result<User, Box<dyn Error>>;
    async fn insert(&self, user: User) -> Result<(), Box<dyn Error>>;
    async fn update(&self, user: User) -> Result<(), Box<dyn Error>>;
}
