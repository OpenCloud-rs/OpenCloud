use crate::lib::db::user::hash_password;
use crate::lib::db::user::model::User;
use datagn::DatabasePool;
use logger::error;
use sqlx::Row;

pub async fn _get_users(database: &mut DatabasePool) -> Vec<User> {
    let response = database
        .execute_and_fetch_all("SELECT * FROM User")
        .await
        .unwrap();
    let mut person_vec: Vec<User> = Vec::new();

    for row in response {
        let name: String = row.try_get("name").expect("Error");
        person_vec.push(User {
            id: row.try_get("id").expect("Error"),
            name: name.clone(),
            password: row.try_get("password").expect("Error"),
            token: row.try_get("token").unwrap_or_default(),
            email: row.try_get("email").unwrap_or_default(),
            home: Some(format!("./home/{}", name)),
        });
    }

    person_vec
}

pub async fn get_user_by_token(database: &mut DatabasePool, token: String) -> Option<User> {
    let query = database
        .execute_and_fetch_all_with_bind("SELECT * FROM User WHERE token = ?1", &[token])
        .await
        .expect("Error");
    let mut user_vec: Vec<User> = Vec::new();
    for row in query {
        let name: String = row.try_get(1).expect("Error");
        user_vec.push(User {
            id: row.try_get(0).expect("Error"),
            name: name.clone(),
            password: row.try_get(2).expect("Error"),
            token: row.try_get(3).unwrap_or_default(),
            email: row.try_get(4).unwrap_or_default(),
            home: Some(format!("./home/{}", name)),
        });
    }

    if let Some(e) = user_vec.get(0) {
        Some(e.clone())
    } else {
        None
    }
}

pub async fn get_id_of_user(
    database: &mut DatabasePool,
    name: String,
    password: String,
) -> Option<i32> {
    let query: i32 = match database
        .execute_and_fetch_one_with_bind(
            "SELECT id FROM User WHERE name=?1 AND password=?2",
            &[name, hash_password(password)],
        )
        .await
    {
        Ok(e) => e.try_get::<i32, &str>("id").unwrap(),
        Err(e) => {
            if cfg!(feature = "log") {
                error(format!("Error on get_id_of_user : {:?}", e));
            }
            -1
        }
    };
    if query == -1 {
        None
    } else {
        Some(query)
    }
}
