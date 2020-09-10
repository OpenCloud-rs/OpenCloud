use crate::lib::db::sqlite_conn::*;
use crate::lib::db::user::model;
use rusqlite::params;
pub fn get_user() {
    let conn = conn();
    println!("{}",conn.prepare("SELECT * FROM User").expect("Error").column_count());
}