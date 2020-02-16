extern crate async_trait;

use crate::model;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait SampleModelRepository {
    async fn get_by_email_pass(
        &self,
        email: String,
        pass: String,
    ) -> Result<model::SampleModel, Box<dyn Error>>;
}
