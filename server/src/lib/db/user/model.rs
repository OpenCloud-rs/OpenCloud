use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub token: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub home: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimalUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

pub struct Id {
    pub id: i32,
}
