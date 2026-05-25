use crate::handlers::FullUserRequest;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

type AppState = Arc<SqlitePool>;

pub async fn user_create_handler(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
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

pub async fn user_update_handler(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
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

    Ok("USer updated".to_string())
}
