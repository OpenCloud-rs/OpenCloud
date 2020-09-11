use crate::lib::db::sqlite_conn::*;
use crate::lib::db::user::model::User;
use rusqlite::params;
pub fn get_user() {
    let conn = conn();
    let mut vec: Vec<User> = Vec::new();
    let mut stmt = conn.prepare("SELECT * FROM User").expect("Can't do prepared request");
    let person_iter = stmt.query_map(params![], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
            token: String::new(),
            email: String::new()
        })
    }).expect("Error on mapping request");

    for person in person_iter {
        vec.push(person.expect("Error"))
    }
}