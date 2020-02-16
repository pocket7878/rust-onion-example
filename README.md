# rust-layered-example

Rust Layered architecture application example.

Separate each layer into crates:

- domain
- data
- application

# Domain

Define model, repository traits, repository provider.

## Model

```rust
#[derive(Debug)]
pub struct SampleModel {
    id: i32
}

impl SampleModel {
    pub fn new(id: i32) -> Self {
        SampleModel { id }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
```

## Repository

Provide repository trait definition.

```rust
#[async_trait]
pub trait SampleModelRepository {
    async fn get_by_email_pass(&self, email: String, pass: String) -> Result<model::SampleModel, Box<dyn Error>>;
}
```

## Provider

Provide repository provider trait.

Use `Box<dyn Repository>` to hide concrete type.

```rust
pub trait SampleModelRepositoryProvider {
    fn provide_sample_model_repository(&self) -> Box<dyn repository::SampleModelRepository>;
}
```

# Data

Implement detail traits.
And provide implementation through provider trait.

## Provider

```rust
pub struct DataModule {}

impl SampleModelRepositoryProvider for DataModule {
    fn provide_sample_model_repository(&self) -> Box<dyn SampleModelRepository> { 
        return Box::new(SampleModelRepositoryImpl::new());
    }
}
```

## Entity

Implement DTO.

```rust
#[derive(Deserialize, Debug)]
pub(crate) struct SampleModelImpl {
    pub(crate) id: i32,
}
```

## Repository

Doing real I/O operations.
And convert results into domain model.

```rust
#[async_trait]
impl repository::SampleModelRepository for SampleModelRepositoryImpl {
    async fn get_by_email_pass(&self, email: String, password: String) -> Result<model::SampleModel, Box<dyn Error>> {
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

        //Convert to domain model.
        Ok(model::SampleModel::new(sample_model.id))
    }
}
```