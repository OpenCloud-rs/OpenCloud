use crate::lib::db::conn::*;
use crate::lib::db::user::model::{Id, User};
use futures::TryStreamExt;
use sqlx::Row;

pub async fn get_users() -> Vec<User> {
    let mut conn = conn().await;
    let mut vec: Vec<User> = Vec::new();
    //let get = sqlx::query_as::<_ User>(sql);
    let mut ll = sqlx::query("SELECT * FROM User").fetch(&mut conn);
    let mut person_vec: Vec<User> = Vec::new();
    while let Some(row) = ll.try_next().await.expect("Error") {
        person_vec.push(
            User {
                id: row.try_get("id").expect("Error"),
                name: row.try_get("name").expect("Error"),
                password: row.try_get("password").expect("Error"),
                token: row.try_get("token").unwrap_or(String::new()),
                email: row.try_get("email").unwrap_or(String::new()),
                home: String::new(),
           }
        );
    };

    for mut person in person_vec {
        person.home = format!("./home/{}/", person.name);
        vec.push(person);
    }
    vec
}
pub async fn get_user_by_token(token: String) -> Option<User> {
    let mut conn = conn().await;
    let mut query = sqlx::query("SELECT * FROM User WHERE token = ?").bind(token).fetch(&mut conn);
    let mut user_vec: Vec<User> = Vec::new();
    while let Some(row) = query.try_next().await.expect("Error") {
        user_vec.push(User {
            id: row.try_get(0).expect("Error"),
            name: row.try_get(1).expect("Error"),
            password: row.try_get(2).expect("Error"),
            token: row.try_get(3).unwrap_or(String::new()),
            email: row.try_get(4).unwrap_or(String::new()),
            home: String::new(),
        });
    };

    let mut result = None;
    for mut user in user_vec {
        user.home = format!("./home/{}", user.name);
        result = Some(user);
        break;
    }

    result
}
pub async fn get_id(name: String, password: String) -> Option<i32> {
    let mut conn = conn().await;
    let mut id: Vec<i32> = Vec::new();
    let mut query_id: Vec<Id> = Vec::new();
    let mut query = sqlx::query("SELECT id FROM User WHERE name=? AND password=?").bind(name).bind(password).fetch(&mut conn);
    while let Some(row) = query.try_next().await.expect("Error") {
        query_id.push(Id { id: row.try_get("id").expect("Error")});
    };
    for ids in query_id {
        id.push(ids.id);
    }
    id.first().cloned()
}
