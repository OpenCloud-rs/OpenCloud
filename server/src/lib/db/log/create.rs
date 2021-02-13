use datagn::DatabasePool;
use logger::error;

pub async fn create(database: &mut DatabasePool) {
    match database.execute("CREATE TABLE IF NOT EXISTS Log (
        id              INTEGER PRIMARY KEY,
        type            TEXT NOT NULL,
        user_id         INTEGER NOT NULL,
        date           TEXT
        )").await {
            Ok(_) => {},
            Err(e) => {
                error(e);
            }
        };
}
