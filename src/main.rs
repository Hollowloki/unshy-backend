use axum::{
    http::{StatusCode, Uri},
    Router,
};
use std::net::SocketAddr;
mod controllers;
mod db;
mod errors;
mod models;
mod router;
mod utils;

#[tokio::main]
async fn main() {
    let api_routes = Router::new().nest("/api/v1", router::routes::routes().await);

    let app = Router::new().merge(api_routes).fallback(fallback);
    models::sync_indexes()
        .await
        .expect("Failed to sync database indexes");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}
