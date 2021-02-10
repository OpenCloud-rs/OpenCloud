use sqlx::{Connection, SqliteConnection};

pub async fn conn() -> SqliteConnection {
    sqlx::sqlite::SqliteConnection::connect("./db.sql")
        .await
        .expect("Error on connect")
}
