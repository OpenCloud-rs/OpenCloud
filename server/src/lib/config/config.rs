use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i64,
    pub folder_root: String,
    pub db_server: String,
    pub db_type: String,
    pub db_port: i64
}

impl Config {
    pub fn get_server(&self) -> String {
        format!("{}:{}", self.server_ip, self.server_port)
    }
    pub fn get_server_ip(&self) -> String {
        self.server_ip.clone()
    }
    pub fn _get_server_port(&self) -> i64 {
        self.server_port.clone()
    }
    pub fn _get_folder_root(&self) -> String {
        self.folder_root.clone()
    }
    pub fn _get_db_server(&self) -> String {
        self.db_server.clone()
    }
    pub fn _get_db_port(&self) -> i64 {
        self.db_port.clone()
    }
    pub fn _get_db_type(&self) -> String {
        self.db_type.clone()
    }

}
