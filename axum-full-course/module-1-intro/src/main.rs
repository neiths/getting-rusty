use axum::{
    Router,
    http::StatusCode,
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

async fn with_status() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Resource created!")
}

async fn conditional_response() -> (StatusCode, &'static str) {
    let is_working = true;

    if is_working {
        (StatusCode::OK, "Everything is working!")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Service is down")
    }
}

async fn echo(body: String) -> String {
    format!("You sent: {}", body)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello", get(hello_axum))
        .route("/health", get(health_check))
        .route("/created", get(with_status))
        .route("/status", get(conditional_response))
        .route("/echo", post(echo));

    println!("🚀 Module 01: Introduction to Axum");
    println!("   Server running on http://localhost:3000");
    println!();
    println!("📝 Try these endpoints:");
    println!("   GET  /        - Hello World");
    println!("   GET  /hello   - Welcome message");
    println!("   GET  /health  - Health check");
    println!("   GET  /created - Status code example");
    println!("   POST /echo    - Echo back your message");
    println!();
    println!("💡 Example: curl http://localhost:3000/hello");
    println!("💡 Example: curl -X POST -d 'Hello!' http://localhost:3000/echo");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
