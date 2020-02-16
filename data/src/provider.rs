use domain::{provider::SampleModelRepositoryProvider, repository::SampleModelRepository};

use crate::repository::SampleModelRepositoryImpl;

pub struct DataModule {}

impl DataModule {
    pub fn new() -> DataModule {
        return DataModule {};
    }
}

impl SampleModelRepositoryProvider for DataModule {
    fn provide_sample_model_repository(&self) -> Box<dyn SampleModelRepository> {
        return Box::new(SampleModelRepositoryImpl::new());
    }
}
