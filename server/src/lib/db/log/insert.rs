use crate::lib::db::sqlite_conn::conn;
use chrono::{NaiveTime, Utc, DateTime, NaiveDateTime};
use chrono::format::Fixed::TimezoneName;
use rusqlite::params;
use crate::lib::db::log::model::action_type;

pub fn insert(user_id: i32, action_type: action_type) {
    let date: String = Utc::now().to_rfc2822();
    let conn = conn();
    conn.execute(
        "INSERT INTO Log (type,user_id,date) VALUES(?1, ?2, ?3);",
        params![action_type.format(), user_id, date],
    ).expect("Error");

}
