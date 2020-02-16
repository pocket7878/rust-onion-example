use crate::repository;

pub trait SampleModelRepositoryProvider {
    fn provide_sample_model_repository(&self) -> Box<dyn repository::SampleModelRepository>;
}
