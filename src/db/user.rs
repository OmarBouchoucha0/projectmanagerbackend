use sqlx::{FromRow, Pool, Sqlite, sqlite::SqliteQueryResult};

#[derive(FromRow)]
pub struct User {
    id: u32,
    firstname: String,
    lastname: String,
    email: String,
    password: String,
}

pub async fn user_init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstname TEXT NOT NULL,
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
    firstname: &str,
    lastname: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO users (firstname,lastname,email,password) VALUES (?,?,?,?)")
        .bind(firstname)
        .bind(lastname)
        .bind(email)
        .bind(password)
        .execute(pool)
        .await
}

pub async fn user_update(
    pool: &Pool<Sqlite>,
    firstname: &str,
    lastname: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users 
        SET firstname = ?, lastname = ?, password = ? 
        WHERE email = ?
        "#,
    )
    .bind(firstname)
    .bind(lastname)
    .bind(email)
    .bind(password)
    .execute(pool)
    .await
}

pub async fn user_exists(
    pool: &Pool<Sqlite>,
    email: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT * from users 
        WHERE email = ? and password = ?
        "#,
    )
    .bind(email)
    .bind(password)
    .fetch_optional(pool)
    .await
}
