use datagn::{config::DatabaseConfig, database::DatabaseType};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server_ip: String,
    pub server_port: i64,
    pub folder_root: String,
    pub db_ip: String,
    pub db_type: DatabaseType,
    pub db_port: Option<i64>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_database: Option<String>,
}

impl Config {
    pub fn get_server(&self) -> String {
        format!("{}:{}", self.get_server_ip(), self.get_server_port())
    }
    pub fn get_server_ip(&self) -> String {
        self.server_ip.clone()
    }
    pub fn get_server_port(&self) -> i64 {
        self.server_port
    }
    pub fn get_db_config(&self) -> datagn::config::DatabaseConfig {
        match self.db_type {
            #[cfg(feature = "sqlite")]
            DatabaseType::Sqlite => DatabaseConfig {
                database_type: DatabaseType::Sqlite,
                ip: self.db_ip.clone(),
                port: String::new(),
                user: String::new(),
                password: String::new(),
                database: None,
            },
            #[cfg(feature = "mysql")]
            DatabaseType::Mysql => DatabaseConfig {
                database_type: DatabaseType::Mysql,
                ip: self.db_ip.clone(),
                port: self.db_port.clone().unwrap_or_default().to_string(),
                user: self.db_user.clone().unwrap_or_default(),
                password: self.db_password.clone().unwrap_or_default(),
                database: self.db_database.clone(),
            },
            #[cfg(feature = "postgres")]
            DatabaseType::Postgresql => DatabaseConfig {
                database_type: DatabaseType::Postgresql,
                ip: self.db_ip.clone(),
                port: self.db_port.clone().unwrap_or_default().to_string(),
                user: self.db_user.clone().unwrap_or_default(),
                password: self.db_password.clone().unwrap_or_default(),
                database: self.db_database.clone(),
            },
            #[cfg(feature = "mssql")]
            DatabaseType::Mssql => DatabaseConfig {
                database_type: DatabaseType::Mssql,
                ip: self.db_ip.clone(),
                port: self.db_port.clone().unwrap_or_default().to_string(),
                user: self.db_user.clone().unwrap_or_default(),
                password: self.db_password.clone().unwrap_or_default(),
                database: self.db_database.clone(),
            },
        }
    }
}
