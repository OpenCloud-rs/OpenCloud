use crate::lib::db::conn::conn;
use chrono::Utc;
use rusqlite::params;
use crate::lib::db::log::model::ActionType;

pub fn insert(user_id: i32, action_type: ActionType) {
    let date: String = Utc::now().to_rfc2822();
    let conn = conn();
    conn.execute(
        "INSERT INTO Log (type,user_id,date) VALUES(?1, ?2, ?3);",
        params![action_type.format(), user_id, date],
    ).expect("Error");

}
