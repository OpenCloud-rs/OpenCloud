use crate::lib::db::conn::conn;
use crate::lib::db::log::model::ActionType;
use chrono::Utc;

pub async fn insert(user_id: i32, action_type: ActionType) {
    let date: String = Utc::now().to_rfc2822();
    let mut conn = conn().await;
    sqlx::query("INSERT INTO Log (type,user_id,date) VALUES(?, ?, ?)")
        .bind(action_type.format())
        .bind(user_id)
        .bind(date)
        .fetch(&mut conn);
}
