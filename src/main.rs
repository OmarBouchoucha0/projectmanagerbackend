use axum::{
    Router,
    routing::{get, post},
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod db;
mod handlers;
use crate::{
    db::db,
    handlers::{user_create, user_exists, user_login, user_register, user_update},
};

async fn test() -> &'static str {
    "Hello from Backend!"
}

#[tokio::main]
async fn main() {
    // debug loggin
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    //  WARNING: this will crash the program probebly not the best of ideas but will do for now
    db(&pool).await.unwrap();
    let state = Arc::new(pool);
    let app = Router::new()
        .route("/api/test", get(test))
        .route("/api/user/create", post(user_create))
        .route("/api/user/update", post(user_update))
        .route("/api/user/exists", post(user_exists))
        .route("/api/user/login", post(user_login))
        .route("/api/user/register", post(user_register))
        .with_state(state)
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("Listening on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();
}
