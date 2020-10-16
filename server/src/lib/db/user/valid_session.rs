use crate::lib::db::conn::conn;
use rusqlite::params;

pub fn valid_session(token: String) -> bool {
    let mut result = false;
    if !token.is_empty() {
        let conn = conn();
        let exec: rusqlite::Result<i32> = conn.query_row_and_then(
            "SELECT * FROM `User` WHERE token = ?1",
            params![token],
            |row| row.get("id"),
        );
        match exec {
            Ok(_) => result = true,
            Err(_) => {}
        }
    }
    result
}
