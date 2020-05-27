use serde::{Deserialize,Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: i64,
    pub folder_root: String,
}