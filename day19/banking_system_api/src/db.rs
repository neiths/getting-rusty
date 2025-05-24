use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type Db = Pool<Sqlite>;

pub async fn init_db() -> Db {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:banking.db")
        .await
        .expect("❌ Could not connect to DB");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("❌ Failed to create table");

    pool
}
