use crate::handlers::{CheckUserRequest, FullUserRequest, FullUserResponse};

use axum::response::IntoResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

type AppState = Arc<SqlitePool>;

// TODO : we are not handling all the cases here will habe to fix
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
        Ok(row) => {
            let user_data = FullUserResponse {
                id: row.get("id"),
                firstname: row.get("firstname"),
                lastname: row.get("lastname"),
                email: row.get("email"),
            };
            (StatusCode::OK, Json(user_data)).into_response()
        }
        Err(e) => {
            // error 2097 is sqllite error for unique constraint failure
            if let Some(sqlx::Error::Database(db_err)) = e.downcast_ref::<sqlx::Error>()
                && db_err.code().as_deref() == Some("2067")
            {
                return (
                    StatusCode::CONFLICT,
                    "An account with this email already exists.",
                )
                    .into_response();
            }
            tracing::error!("Database error during login check: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
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
) -> impl IntoResponse {
    let user = crate::db::user::user_exists(&pool, &payload.email, &payload.password).await;
    match user {
        Ok(_) => (StatusCode::OK, "User Exists").into_response(),
        Err(e) => {
            if let Some(sqlx::Error::RowNotFound) = e.downcast_ref::<sqlx::Error>() {
                return (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response();
            }
            tracing::error!("Database error during login check: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}

pub async fn user_login_handler(
    State(pool): State<AppState>,
    Json(payload): Json<CheckUserRequest>,
) -> impl IntoResponse {
    let user = crate::db::user::user_exists(&pool, &payload.email, &payload.password).await;

    match user {
        Ok(row) => {
            let user_data = FullUserResponse {
                id: row.get("id"),
                firstname: row.get("firstname"),
                lastname: row.get("lastname"),
                email: row.get("email"),
            };
            (StatusCode::OK, Json(user_data)).into_response()
        }
        Err(e) => {
            if let Some(sqlx::Error::RowNotFound) = e.downcast_ref::<sqlx::Error>() {
                return (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response();
            }
            tracing::error!("Database error during login check: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}
