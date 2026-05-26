use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use rand::rngs::OsRng;
use sqlx::{
    FromRow, Pool, Row, Sqlite,
    sqlite::{SqliteQueryResult, SqliteRow},
};

#[derive(FromRow)]
pub struct User {
    id: u32,
    firstname: String,
    lastname: String,
    email: String,
    password_hash: String,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub async fn user_init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstname TEXT NOT NULL,
            lastname TEXT NOT NULL,
            passwordHash TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
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
) -> Result<SqliteQueryResult> {
    let password_hash = hash_password(password).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let query =
        sqlx::query("INSERT INTO users (firstname,lastname,email,passwordHash) VALUES (?,?,?,?)")
            .bind(firstname)
            .bind(lastname)
            .bind(email)
            .bind(password_hash)
            .execute(pool)
            .await?;
    Ok(query)
}

pub async fn user_update(
    pool: &Pool<Sqlite>,
    firstname: &str,
    lastname: &str,
    email: &str,
    password: &str,
) -> Result<SqliteQueryResult> {
    let password_hash = hash_password(password).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let query = sqlx::query(
        r#"
        UPDATE users 
        SET firstname = ?, lastname = ?, passwordHash = ? 
        WHERE email = ?
        "#,
    )
    .bind(firstname)
    .bind(lastname)
    .bind(email)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(query)
}

pub async fn user_exists(pool: &Pool<Sqlite>, email: &str, password: &str) -> Result<SqliteRow> {
    let query = sqlx::query(
        r#"
        SELECT * from users 
        WHERE email = ? 
        "#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;
    let stored_hash: String = query.try_get("passwordHash")?;
    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {}", e))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Invalid password"))?;

    Ok(query)
}
