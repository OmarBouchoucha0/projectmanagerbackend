use anyhow::Result;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteQueryResult, SqliteRow},
};

pub async fn project_init(pool: &Pool<Sqlite>) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            workspace_id INTEGER NOT NULL,
            favorite BOOLEAN NOT NULL DEFAULT 0,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (workspace_id) REFERENCES users(id) ON DELETE CASCADE,
            UNIQUE(name, user_id,workspace_id)
        )
        "#,
    )
    .execute(pool)
    .await
}

pub async fn project_create(
    pool: &Pool<Sqlite>,
    name: &str,
    user_email: &str,
    workspace_id: &str,
) -> Result<SqliteRow> {
    let row = sqlx::query(
        r#"
        INSERT INTO workspaces (name, user_id, workspace_id) 
        VALUES (?, (SELECT id FROM users WHERE email=?) 
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(user_email)
    .bind(workspace_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn project_favorite(
    pool: &Pool<Sqlite>,
    name: &str,
    user_email: &str,
    workspace_id: &str,
) -> Result<SqliteRow> {
    let row = sqlx::query(
        r#"
        UPDATE workspaces 
        SET favorite = ?
        WHERE name = ? 
        AND user_id = (SELECT id FROM users WHERE email = ?)
        AND workspace_id = ?
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(user_email)
    .bind(workspace_id)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
