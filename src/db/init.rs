use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

use crate::db::user::user_init;

pub async fn init_db(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    user_init(pool).await
}
