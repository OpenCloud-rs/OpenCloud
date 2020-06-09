use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i64,
    pub client_ip: String,
    pub client_port: i64,
    pub folder_root: String,
}

impl Config {
    pub(crate) fn get_server_ip(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }
    pub(crate) fn get_client_ip(&self) -> String {
        format!("{}:{}", self.client_ip, self.client_port)
    }
}
