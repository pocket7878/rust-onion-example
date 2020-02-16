extern crate clap;
extern crate data;
extern crate domain;
extern crate futures;
extern crate serde_yaml;
extern crate tokio;

use clap::App;
use domain::provider::SampleModelRepositoryProvider;

#[tokio::main]
async fn main() {
    App::new("rust-layered-example")
        .version("1.0")
        .author("Pocket7878 <poketo7878@gmail.com>")
        .get_matches();

    let data_module = data::DataModule::new();

    let repo = data_module.provide_sample_model_repository();
    let model = repo
        .get_by_email_pass("my_email".to_owned(), "my_pass".to_owned())
        .await;
    println!("{:?}", model);
}
