pub mod user;
use axum::response::IntoResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

use crate::handlers::user::{user_create_handler, user_exists_handler, user_login_handler};
use crate::handlers::user::{user_register_handler, user_update_handler};

type AppState = Arc<SqlitePool>;

#[derive(Deserialize)]
pub struct FullUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CheckUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct FullUserResponse {
    pub id: u32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

pub async fn user_create(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> impl IntoResponse {
    user_create_handler(State(pool), Json(payload)).await
}

pub async fn user_update(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> Result<String, (StatusCode, String)> {
    user_update_handler(State(pool), Json(payload)).await
}

pub async fn user_exists(
    State(pool): State<AppState>,
    Json(payload): Json<CheckUserRequest>,
) -> impl IntoResponse {
    user_exists_handler(State(pool), Json(payload)).await
}

pub async fn user_login(
    State(pool): State<AppState>,
    Json(payload): Json<CheckUserRequest>,
) -> impl IntoResponse {
    user_login_handler(State(pool), Json(payload)).await
}

pub async fn user_register(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> impl IntoResponse {
    user_register_handler(State(pool), Json(payload)).await
}
