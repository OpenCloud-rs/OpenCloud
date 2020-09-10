use rusqlite::Connection;


pub fn conn() -> Connection {
    Connection::open("./db.sql").expect("Can't open the file")
}