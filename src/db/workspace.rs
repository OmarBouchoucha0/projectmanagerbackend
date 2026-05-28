use anyhow::Result;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteQueryResult, SqliteRow},
};

pub async fn workspace_init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workspaces (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            UNIQUE(name, user_id)
        )
        "#,
    )
    .execute(pool)
    .await
}

pub async fn workspace_create(
    pool: &Pool<Sqlite>,
    name: &str,
    user_email: &str,
) -> Result<SqliteRow> {
    let row = sqlx::query(
        r#"
        INSERT INTO workspaces (name, user_id) 
        VALUES (?, (SELECT id FROM users WHERE email=?))
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(user_email)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
