use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i64,
    pub folder_root: String,
}

impl Config {
    pub(crate) fn get_server_ip(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }
}
