#[derive(Debug)]
pub struct SampleModel {
    id: i32,
}

impl SampleModel {
    pub fn new(id: i32) -> Self {
        SampleModel { id }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
