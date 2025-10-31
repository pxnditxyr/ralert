use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use ralert::{
    infrastructure::{
        config::environments::ENVIRONMENTS,
        di::container::Container,
        web::app_state::AppState
    },
    presentation::web::health_handler::health_handler,
};


#[tokio::main]
async fn main() {
    println!("Hello to Ralert!!! 🚀 by Pxndxs 🐼");
    println!("Server running on port: {}", ENVIRONMENTS.server_port);

    let container = Container::new();

    let app_state = Arc::new(AppState {
        health_check_use_case: container.health_check_uc,
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], ENVIRONMENTS.server_port));
    println!("🚀 Server running on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
