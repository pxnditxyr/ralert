use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router, middleware,
    routing::{get, post},
};
use ralert::{
    infrastructure::{
        config::environments::ENVIRONMENTS,
        database::connection::create_pool,
        di::container::Container,
        middleware::jwt_middleware::{admin_only_middleware, jwt_auth_middleware},
        web::app_state::AppState,
    },
    presentation::web::{
        auth_handler::sign_in_handler,
        health_handler::health_handler,
        user_handler::{admin_only_handler, get_profile_handler},
    },
};

#[tokio::main]
async fn main() {
    println!("Hello to Ralert!!! 🚀 by Pxndxs 🐼");
    println!("🚀 Starting up the server xD");
    println!("Server running on port: {}", ENVIRONMENTS.server_port);

    let pool = create_pool()
        .await
        .expect("FATAL_ERROR: Database connection failed!");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("FATAL_ERROR: Database migration failed!");

    println!("✅ Database migration completed!");

    let container = Container::new(pool);

    let app_state = Arc::new(AppState {
        health_check_use_case: container.health_check_uc,
        sign_in_use_case: container.sign_in_uc,
        get_user_profile_use_case: container.get_profile_uc,
    });

    let public_routes = Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/auth/sign-in", post(sign_in_handler));

    let protected_routes = Router::new()
        .route("/api/v1/profile", get(get_profile_handler))
        .layer(middleware::from_fn(jwt_auth_middleware));

    let admin_routes = Router::new()
        .route("/api/v1/admin/test", get(admin_only_handler))
        .layer(middleware::from_fn(admin_only_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], ENVIRONMENTS.server_port));
    println!("🚀 Server running on: {addr}");
    println!("\n📋 Available endpoints:");
    println!("  PUBLIC:");
    println!("    GET  /health");
    println!("    POST /api/v1/auth/sign-in");
    println!("  PROTECTED (requires JWT):");
    println!("    GET  /api/v1/profile");
    println!("  ADMIN ONLY:");
    println!("    GET  /api/v1/admin/test\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
