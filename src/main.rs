use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use ralert::{
    application::use_cases::health_check_uc_impl::HealthCheckUseCaseImpl,
    infrastructure::config::environments::ENVIRONMENTS,
    presentation::web::health_handler::health_handler,
    AppState
};


#[tokio::main]
async fn main() {
    println!("Hello to Ralert!!! 🚀 by Pxndxs 🐼");
    println!("Server running on port: {}", ENVIRONMENTS.server_port);

    let health_check_uc_impl = Arc::new(HealthCheckUseCaseImpl::new());

    let app_state = Arc::new(AppState {
        health_check_use_case: health_check_uc_impl,
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], ENVIRONMENTS.server_port));
    println!("🚀 Server running on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
