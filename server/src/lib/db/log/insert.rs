use crate::lib::db::log::model::ActionType;
use chrono::Utc;

use datagn::DatabasePool;

pub async fn insert(database: &mut DatabasePool, user_id: i32, action_type: ActionType) {
    let date: String = Utc::now().to_rfc2822();
    database.execute_with_bind("INSERT INTO Log (type,user_id,date) VALUES(?1, ?2, ?3)", &[action_type.format(),user_id.to_string(),date.clone()]).await.expect("Error");
}
