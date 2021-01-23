use crate::lib::db::user::hash_password;
use crate::lib::db::user::model::User;
use crate::lib::{db::conn::*, log::log::error};
use futures::TryStreamExt;
use sqlx::Row;

pub async fn get_users() -> Vec<User> {
    let mut conn = conn().await;
    let mut response = sqlx::query("SELECT * FROM User").fetch(&mut conn);
    let mut person_vec: Vec<User> = Vec::new();
    while let Ok(Some(row)) = response.try_next().await {
        person_vec.push(User {
            id: row.try_get("id").expect("Error"),
            name: row.try_get("name").expect("Error"),
            password: row.try_get("password").expect("Error"),
            token: row.try_get("token").unwrap_or(String::new()),
            email: row.try_get("email").unwrap_or(String::new()),
            home: format!(
                "./home/{}/",
                row.try_get::<&str, &str>("name").expect("Error")
            ),
        });
    }

    person_vec
}
pub async fn get_user_by_token(token: String) -> Option<User> {
    let mut conn = conn().await;
    let mut query = sqlx::query("SELECT * FROM User WHERE token = ?")
        .bind(token)
        .fetch(&mut conn);
    let mut user_vec: Vec<User> = Vec::new();
    while let Ok(Some(row)) = query.try_next().await {
        user_vec.push(User {
            id: row.try_get(0).expect("Error"),
            name: row.try_get(1).expect("Error"),
            password: row.try_get(2).expect("Error"),
            token: row.try_get(3).unwrap_or(String::new()),
            email: row.try_get(4).unwrap_or(String::new()),
            home: format!("./home/{}", row.try_get::<&str, usize>(1).expect("Error")),
        });
    }

    if let Some(e) = user_vec.get(0) {
        Some(e.clone())
    } else {
        None
    }
}

pub async fn get_id_of_user(name: String, password: String) -> Option<i32> {
    let mut conn = conn().await;
    let query: (i32,) = match sqlx::query_as("SELECT id FROM User WHERE name=? AND password=?")
        .bind(name)
        .bind(hash_password(password))
        .fetch_one(&mut conn)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            error(format!("Error on get_id_of_user : {:?}", e));
            (-1,)
        }
    };

    Some(query.0)
}
