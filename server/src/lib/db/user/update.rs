use crate::lib::db::conn::conn;
use sqlx::Executor;

pub async fn update_token(token: String, id: i32) -> bool {
    let mut conn = conn().await;

    match conn
        .execute(format!("UPDATE User SET token=\"{}\" WHERE id=\"{}\"", token, id).as_ref())
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
