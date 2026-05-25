use crate::handlers::{CheckUserRequest, ExistsResponse, FullUserRequest};

use axum::response::IntoResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

type AppState = Arc<SqlitePool>;

// TODO : we are no handling all the cases here will habe to fix
pub async fn user_create_handler(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> impl IntoResponse {
    let query = crate::db::user::user_create(
        &pool,
        &payload.firstname,
        &payload.lastname,
        &payload.email,
        &payload.password,
    )
    .await;

    match query {
        Ok(_result) => (StatusCode::OK, "User created successfully").into_response(),
        Err(_e) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }
}

pub async fn user_update_handler(
    State(pool): State<AppState>,
    Json(payload): Json<FullUserRequest>,
) -> Result<String, (StatusCode, String)> {
    crate::db::user::user_update(
        &pool,
        &payload.firstname,
        &payload.lastname,
        &payload.email,
        &payload.password,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok("User updated".to_string())
}

pub async fn user_exists_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CheckUserRequest>,
) -> Result<Json<ExistsResponse>, (StatusCode, String)> {
    let user = crate::db::user::user_exists(&pool, &payload.email, &payload.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(ExistsResponse {
        exists: user.is_some(),
    }))
}
