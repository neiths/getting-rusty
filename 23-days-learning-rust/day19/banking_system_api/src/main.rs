mod db;
mod models;
mod auth;
mod routes;

use axum::{Router};
use routes::{auth_routes, protected_routes};
use db::init_db;

#[tokio::main]
async fn main() {
    let pool = init_db().await;

    let app = Router::new()
        .merge(auth_routes(pool.clone()))
        .merge(protected_routes(pool.clone()));

    println!("🚀 Server running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}