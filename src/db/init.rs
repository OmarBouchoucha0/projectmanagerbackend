use sqlx::{Pool, Sqlite};

use crate::db::{project::project_init, user::user_init, workspace::workspace_init};

pub async fn init_db(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    user_init(pool).await?;
    workspace_init(pool).await?;
    project_init(pool).await?;
    Ok(())
}
