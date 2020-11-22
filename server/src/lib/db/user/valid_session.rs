use crate::lib::db::conn::conn;
use sqlx::Executor;

pub async fn valid_session(token: String) -> bool {
    let mut result = false;
    if !token.is_empty() {
        let mut conn = conn().await;
        let query = conn
            .execute(format!("SELECT * FROM `User` WHERE token = \"{}\"", token).as_ref())
            .await;
        match query {
            Ok(_) => result = true,
            Err(_) => {}
        };
    }
    result
}
