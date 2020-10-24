use crate::lib::db::conn::conn;

pub fn create() {
    let conn = conn();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Log (
                  id              INTEGER PRIMARY KEY,
                  type            TEXT NOT NULL,
                  user_id         INTEGER NOT NULL,
                  date           TEXT
                  )",
        "".bytes(),
    )
    .expect("Error");
}
