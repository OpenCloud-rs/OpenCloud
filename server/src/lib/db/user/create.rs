use sqlx::Executor;

use crate::lib::db::conn::conn;

pub async fn create() {
    let mut conn = conn().await;
    match conn
        .execute(
            "CREATE TABLE IF NOT EXISTS User (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  password        TEXT NOT NULL,
                  token           TEXT,
                  email           TEXT
                  )",
        )
        .await
    {
        Ok(_) => {}
        Err(_) => {
            error("Error on create the database")
        }
    }
}
