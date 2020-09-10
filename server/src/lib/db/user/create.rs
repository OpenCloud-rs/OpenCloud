use crate::lib::db::sqlite_conn::conn;

pub fn create() {
	let conn = conn();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS User (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  password            BLOB
                  )",
		"".bytes()
    ).expect("Error");
}