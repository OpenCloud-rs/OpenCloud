use crate::lib::db::conn::conn;
use sqlx::Executor;

pub async fn update_token(token: String, id: i32) {
    let mut conn = conn().await;
    
    //conn.prepare(r#"UPDATE User SET token=? WHERE id=?"#).bind(token).bind(id).await;
    conn.execute(format!("UPDATE User SET token=\"{}\" WHERE id=\"{}\"", token, id).as_ref()).await.expect("Error");
    //conn.execute(r#"UPDATE User SET token=? WHERE id=?"#).bind(token).bind(id)
    //let query = sqlx::query(r#"UPDATE User SET token=? WHERE id=?"#).bind(token).bind(id).fetch(&mut conn).await.expect("Error");
}
