use sqlx::{Pool, Sqlite, sqlite::SqliteQueryResult};

pub async fn user_init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            lastname TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await
}

pub async fn user_create(
    pool: &Pool<Sqlite>,
    name: &str,
    lastname: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO users (name,email,password) VALUES (?,?,?)")
        .bind(name)
        .bind(lastname)
        .bind(email)
        .bind(password)
        .execute(pool)
        .await
}

pub async fn user_update(
    pool: &Pool<Sqlite>,
    name: &str,
    lastname: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users 
        SET name = ?, lastname = ?, password = ? 
        WHERE email = ?
        "#,
    )
    .bind(name)
    .bind(lastname)
    .bind(email)
    .bind(password)
    .execute(pool)
    .await
}
