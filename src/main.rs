use axum::{Router, extract::State, routing::get};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

type AppState = Arc<SqlitePool>;

async fn test() -> &'static str {
    "Hello from Backend!"
}

async fn create_user(State(pool): State<AppState>) -> String {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("test user")
        .execute(&*pool)
        .await
        .unwrap();

    "User created".to_string()
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::query(
        r#"
    CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    )
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let state = Arc::new(pool);

    let app = Router::new()
        .route("/api/test", get(test))
        .route("/api/create-user", get(create_user))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
