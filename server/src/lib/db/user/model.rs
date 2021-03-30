use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub token: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub password: String,
    pub home: Option<String>,
}
