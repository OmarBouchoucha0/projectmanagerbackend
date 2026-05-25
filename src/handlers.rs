pub mod user;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

use crate::handlers::user::user_create_handler;
use crate::handlers::user::user_update_handler;

type AppState = Arc<SqlitePool>;

#[derive(Deserialize)]
pub struct FullUserRequest {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

pub async fn user_create(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> Result<String, (StatusCode, String)> {
    user_create_handler(State(pool), Json(payload)).await
}

pub async fn user_update(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> Result<String, (StatusCode, String)> {
    user_update_handler(State(pool), Json(payload)).await
}
