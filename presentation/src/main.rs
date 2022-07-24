use clap::App;

#[tokio::main]
async fn main() {
    // TODO: Implement web server.
    App::new("rust-layered-example")
        .version("1.0")
        .author("Pocket7878 <poketo7878@gmail.com>")
        .get_matches();

    let infra_provider = infra::Provider::new();
    let create_task_usecase = usecase::task::CreateTaskUseCase::new(infra_provider.provide_task_repository());

    let task_id = create_task_usecase.execute("task name", time::Date::from_calendar_date(2021, time::Month::January, 1).unwrap(), 42).await.unwrap();
    eprintln!("task id: {}", task_id);
}
