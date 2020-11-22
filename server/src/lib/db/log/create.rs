use sqlx::Executor;

use crate::lib::db::conn::conn;

pub async fn create() {
    let mut conn = conn().await;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Log (
                  id              INTEGER PRIMARY KEY,
                  type            TEXT NOT NULL,
                  user_id         INTEGER NOT NULL,
                  date           TEXT
                  )",
    )
    .await
    .expect("Error");
}
