use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::{post, get},
    Router
};
use uuid::Uuid;

use crate::{db::Db, models::{RegisterRequest, LoginRequest, User}, auth::{hash_password, verify_password}};
use sqlx::query_as;

pub fn auth_routes(db: Db) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(db)
}

pub fn protected_routes(db: Db) -> Router {
    Router::new()
        .route("/me", get(me))
        .with_state(db)
}

async fn register(
    State(db): State<Db>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: payload.username.clone(),
        password_hash: hash_password(&payload.password),
    };

    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.password_hash)
    .execute(&db)
    .await;

    match result {
        Ok(_) => Ok(Json(user)),
        Err(_) => Err((StatusCode::CONFLICT, "Username already exists".into())),
    }
}

async fn login(
    State(db): State<Db>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user: Option<User> = query_as("SELECT * FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?;

    match user {
        Some(u) if verify_password(&payload.password, &u.password_hash) => Ok(Json(u)),
        _ => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into())),
    }
}

async fn me() -> &'static str {
    "✅ Authenticated endpoint reached!"
}
