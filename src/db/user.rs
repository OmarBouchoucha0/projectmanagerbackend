use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

pub async fn init_user(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
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

pub async fn create_user(
    pool: &Pool<Sqlite>,
    name: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO users (name,email,password) VALUES (?,?,?)")
        .bind(name)
        .bind(email)
        .bind(password)
        .execute(pool)
        .await
}
