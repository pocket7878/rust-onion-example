use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct SampleModelImpl {
    pub(crate) id: i32,
}
