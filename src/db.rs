pub mod init;
pub mod user;
use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

use crate::db::init::init_db;

pub async fn db(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult> {
    init_db(pool).await.context("failed to initilize db")
}
