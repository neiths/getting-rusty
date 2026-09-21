use axum::{
    Router,
    routing::{get, post},
};

async fn hello_world() -> &'static str {
    "hello, World"
}

async fn hello_axum() -> String {
    format!("Welcome to axum {}!", env! {"CARGO_PKG_VERSION"})
}

async fn health_check() -> &'static str {
    "Ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello", get(hello_axum))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
