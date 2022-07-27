mod api_error;
mod di_container;
mod handler;

use axum::{
    extract::Extension,
    routing::{get, patch, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // initialize infra provider
    let infra_provider = infra::Provider::new()
        .await
        .expect("Failed to initialize infra provider");
    let di_container = Arc::new(di_container::DiContainer::new(infra_provider));

    // build our application with a route
    let app = Router::new()
        .route("/tasks", get(handler::list_tasks::list_tasks))
        .route("/tasks", post(handler::create_task::create_task))
        .route("/tasks/:id", get(handler::fetch_task::fetch_task))
        .route("/tasks/:id", patch(handler::update_task::update_task))
        .route(
            "/tasks/:id/postpone",
            patch(handler::postpone_task::postpone_task),
        )
        .layer(Extension(di_container));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
