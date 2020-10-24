use crate::lib::db::conn::conn;
use rusqlite::params;

pub fn insert_user(
    name: String,
    email: String,
    password: String,
) -> std::result::Result<usize, rusqlite::Error> {
    let conn = conn();
    conn.execute(
        "INSERT INTO User (name,email, password) VALUES(?1, ?2, ?3);",
        params![name, email, password],
    )
    .expect("Error");

    Ok(usize::from(true))
}
