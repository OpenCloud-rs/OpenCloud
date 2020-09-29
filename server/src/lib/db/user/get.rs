use crate::lib::db::sqlite_conn::*;
use crate::lib::db::user::model::{Id, User};
use rusqlite::params;
pub fn get_users() -> Vec<User> {
    let conn = conn();
    let mut vec: Vec<User> = Vec::new();
    let mut stmt = conn
        .prepare("SELECT * FROM User")
        .expect("Can't do prepared request");
    let person_iter = stmt
        .query_map(params![], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
                token: row.get(3).unwrap_or(String::new()),
                email: row.get(4).unwrap_or(String::new()),
                home: String::new(),
            })
        })
        .expect("Error on mapping request");

    for person in person_iter {
        let mut person  = person.expect("Error");
        person.home = format!("./home/{}/", person.name);
        vec.push(person);
    }
    vec
}
pub fn get_user_by_token(token: String) -> Option<User> {
    let conn = conn();
    let mut stmt = conn
        .prepare("SELECT * FROM User WHERE token = ?1")
        .expect("Can't do prepared request");
    let user = stmt.query_map(params![token], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
            token: row.get(3).unwrap_or(String::new()),
            email: row.get(4).unwrap_or(String::new()),
            home: String::new(),
        })
    }).expect("Error");
    let mut result = None;
   for u in user {
       let mut user = u.expect("Error");
       user.home = format!("./home/{}", user.name);
       result  = Some(user);
       break;
    };

    result
}
pub fn get_id(name: String, password: String) -> i32 {
    let conn = conn();
    let mut id: Vec<i32> = Vec::new();
    let mut stmt = conn
        .prepare("SELECT id FROM User WHERE name=?1 AND password=?2")
        .expect("Can't do prepared request");
    let person_iter = stmt
        .query_map(params![name, password], |row| Ok(Id { id: row.get(0)? }))
        .expect("Error on mapping request");
    for ids in person_iter {
        id.push(ids.expect("Error").id);
    }
    id.first().unwrap().to_owned()
}
