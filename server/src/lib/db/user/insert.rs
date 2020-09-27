use crate::lib::db::sqlite_conn::conn;
use rusqlite::params;
use serde::de::Unexpected::Bool;

pub fn insert_user(
    name: String,
    email: String,
    password: String,
) -> std::result::Result<usize, rusqlite::Error> {
    let conn = conn();
    conn.execute(
        "INSERT INTO User (name,email, password) VALUES(?1, ?2, ?3);",
        params![name, email, password],
    );
    
    Ok(usize::from(true))
}
