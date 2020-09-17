use crate::lib::db::sqlite_conn::conn;
use rusqlite::params;

pub fn update_token(token: String, id: i32) {
    let conn = conn();
    conn.execute(
        "UPDATE \"User\"
SET token=?1
WHERE id=?2;",
        params![token, id],
    )
    .expect("Error");
}
