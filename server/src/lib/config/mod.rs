use serde::{Deserialize, Serialize};
use datagn::{config::DatabaseConfig, database::DatabaseType};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i64,
    pub folder_root: String,
    pub db_ip: String,
    pub db_type: DatabaseType,
    pub db_port: Option<i64>,
    pub db_user: Option<String>,
    pub db_password: Option<String>
}

impl Config {
    pub fn get_server(&self) -> String {
        format!("{}:{}", self.get_server_ip(), self.get_server_port())
    }
    pub fn get_server_ip(&self) -> String {
        self.server_ip.clone()
    }
    pub fn get_server_port(&self) -> i64 {
        self.server_port.clone()
    }
    pub fn _get_folder_root(&self) -> String {
        self.folder_root.clone()
    }
    pub fn _get_db_ip(&self) -> String {
        self.db_ip.clone()
    }
    pub fn _get_db_port(&self) -> Option<i64> {
        self.db_port.clone()
    }
    pub fn _get_db_type(&self) -> DatabaseType {
        self.db_type.clone()
    }
    pub fn get_db_config(&self) -> datagn::config::DatabaseConfig {
        match self.db_type {
            DatabaseType::Sqlite => {
                DatabaseConfig {
                    database_type:  DatabaseType::Sqlite,
                    ip: self.db_ip.clone(),
                    port: String::new(),
                    user: String::new(),
                    password: String::new(),
                }
            }
            DatabaseType::Mysql => {
                DatabaseConfig {
                    database_type:  DatabaseType::Mysql,
                    ip: self.db_ip.clone(),
                    port: self.db_port.clone().unwrap().to_string(),
                    user: self.db_user.clone().unwrap(),
                    password: self.db_password.clone().unwrap(),
                }
            }
            DatabaseType::Postgresql => {
                DatabaseConfig {
                    database_type:  DatabaseType::Postgresql,
                    ip: self.db_ip.clone(),
                    port: self.db_port.clone().unwrap().to_string(),
                    user: self.db_user.clone().unwrap(),
                    password: self.db_password.clone().unwrap(),
                }
            }
        }
    }
}