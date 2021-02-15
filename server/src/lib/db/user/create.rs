use datagn::DatabasePool;

pub async fn create(database: &mut DatabasePool) {
    database
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
        .expect("Error");
}
