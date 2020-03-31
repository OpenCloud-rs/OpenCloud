use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub result: bool,
    pub lenght: i64,
    pub content: Vec<String>,
}
