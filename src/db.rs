pub mod init;
pub mod project;
pub mod user;
pub mod workspace;
use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite};

use crate::db::init::init_db;

pub async fn db(pool: &Pool<Sqlite>) -> Result<()> {
    init_db(pool).await.context("failed to initilize db")
}
