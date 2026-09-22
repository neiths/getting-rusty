use axum::{
    Router,
    extract::{Path, Query},
    routing::{delete, get, patch, post, put},
};

/// single path parameter
async fn get_user(Path(id): Path<u64>) -> String {
    format!("Getting user with ID: {}", id)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users/{id}", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
