extern crate async_trait;
extern crate domain;
extern crate reqwest;

use crate::entity;
use async_trait::async_trait;
use domain::model;
use domain::repository;
use std::error::Error;

pub struct SampleModelRepositoryImpl {}

impl SampleModelRepositoryImpl {
    pub fn new() -> Self {
        SampleModelRepositoryImpl {}
    }
}

#[async_trait]
impl repository::SampleModelRepository for SampleModelRepositoryImpl {
    async fn get_by_email_pass(
        &self,
        email: String,
        password: String,
    ) -> Result<model::SampleModel, Box<dyn Error>> {
        /*
        let client = reqwest::Client::new();
        let params = json!({
            "user": {
                "email": email,
                "password": password,
            },
        });
        let res = client.post("https://api.example.com/user/login.json")
            .json(&params)
            .send()
            .await?
            .error_for_status()?;

        let sample_model: entity::SampleModelImpl = res.json().await?;

        //Ok(model::SampleModel::new(sample_model.id))
        */
        Ok(model::SampleModel::new(1))
    }
}
