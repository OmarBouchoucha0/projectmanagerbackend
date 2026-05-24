use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

use crate::db::user::init_user;

pub async fn init_db(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    init_user(pool).await
}
