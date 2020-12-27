use crate::lib::db::conn::*;
use crate::lib::db::user::model::User;
use futures::TryStreamExt;
use sqlx::Row;

pub async fn get_users() -> Vec<User> {
    let mut conn = conn().await;
    let mut vec: Vec<User> = Vec::new();
    let mut response = sqlx::query("SELECT * FROM User").fetch(&mut conn);
    let mut person_vec: Vec<User> = Vec::new();
    while let Ok(Some(row)) = response.try_next().await {
        person_vec.push(User {
            id: row.try_get("id").expect("Error"),
            name: row.try_get("name").expect("Error"),
            password: row.try_get("password").expect("Error"),
            token: row.try_get("token").unwrap_or(String::new()),
            email: row.try_get("email").unwrap_or(String::new()),
            home: String::new(),
        });
    }

    for mut person in person_vec {
        person.home = format!("./home/{}/", person.name);
        vec.push(person);
    }
    vec
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
            home: String::new(),
        });
    }

    let mut result = None;
    for mut user in user_vec {
        user.home = format!("./home/{}", user.name);
        result = Some(user);
        break;
    }

    result
}

pub async fn get_id_of_user(name: String, password: String) -> Option<i32> {
    let mut conn = conn().await;
    let query: (i32,) = match sqlx::query_as("SELECT id FROM User WHERE name=? AND password=?")
        .bind(name)
        .bind(password)
        .fetch_one(&mut conn)
        .await {
            Ok(e) => {
                e
            },
            Err(e) => {
                eprint!("Error on get_id_of_user : {:?}", e);
                (-1, )
            }
        };
        

    Some(query.0)
}
