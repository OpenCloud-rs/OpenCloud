use crate::lib::db::sqlite_conn::conn;
use rusqlite::{params, Error};
pub fn delete(id: i32) -> bool {
    let conn = conn();
    match conn.execute(
        format!("delete from log where id = {}", id).as_str(),
        params![]
    ) {
        Ok(_) => {true}
        Err(_) => {false}
    }
}