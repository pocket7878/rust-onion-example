extern crate serde;
#[macro_use]
extern crate serde_json;

mod entity;
mod provider;
mod repository;

pub use provider::DataModule;
