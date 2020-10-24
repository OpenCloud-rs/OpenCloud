use crate::lib::db::conn::conn;
use rusqlite::params;
pub fn _delete(id: i32) -> bool {
    let conn = conn();
    match conn.execute(
        format!("delete from log where id = {}", id).as_str(),
        params![],
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}
