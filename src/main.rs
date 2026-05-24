use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod db;
use crate::db::db;

type AppState = Arc<SqlitePool>;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

async fn test() -> &'static str {
    "Hello from Backend!"
}

async fn user_create_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<String, (StatusCode, String)> {
    crate::db::user::user_create(
        &pool,
        &payload.name,
        &payload.lastname,
        &payload.email,
        &payload.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("User created".to_string())
}

async fn user_update_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<String, (StatusCode, String)> {
    crate::db::user::user_update(
        &pool,
        &payload.name,
        &payload.lastname,
        &payload.email,
        &payload.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("User created".to_string())
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    //  WARNING: this will crash the program probebly not the best of ideas but will do for now
    db(&pool).await.unwrap();
    let state = Arc::new(pool);

    let app = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/create", post(user_create_handler))
        .route("/api/user/update", post(user_update_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    println!("Listening on http://localhost:5000");

    axum::serve(listener, app).await.unwrap();
}
