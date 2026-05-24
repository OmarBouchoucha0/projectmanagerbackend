use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

pub async fn init_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
            password TEXT NOT NULL
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
}
