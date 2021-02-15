use datagn::database::DatabaseType;
use logger::{error, info};
use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::lib::config::Config;
use std::process::exit;

pub fn default() -> Config {
    let mut vec: Vec<String> = Vec::new();
    let rd = read_dir(PathBuf::from("./")).expect("Error: Can't read the folder");
    for rde in rd {
        let de = rde.expect("Error: Can't read Dir Entry");
        vec.push(de.file_name().into_string().expect("Error: Bad name"));
    }

    if !vec.contains(&String::from("temp")) {
        std::fs::create_dir("./temp").expect("Failed to create the temp folder");
    }
    if !vec.contains(&String::from("db.sql")) {
        std::fs::File::create("./db.sql").expect("Failed to create the database");
    }
    if !vec.contains(&String::from("home")) {
        std::fs::create_dir("./home").expect("Failed to create the home folder");
    }
    if vec.contains(&String::from("config.yaml")) {
        let mut buf = String::new();
        match File::open("./config.yaml") {
            Ok(mut e) => match e.read_to_string(&mut buf) {
                Ok(_) => {}
                Err(_) => {
                    if cfg!(feature = "log") {
                        error("Can't read the config");
                    }
                    exit(1)
                }
            },
            Err(_) => {
                if cfg!(feature = "log") {
                    error("Can't found the config");
                }
                exit(1)
            }
        };
        match serde_yaml::from_str(&buf) {
            Ok(o) => o,
            Err(_) => {
                if cfg!(feature = "log") {
                    error("Config Error");
                }
                exit(1);
            }
        }
    } else {
        let config = Config {
            server_ip: "0.0.0.0".to_string(),
            server_port: 8081,
            folder_root: "/".to_string(),
            db_type: DatabaseType::Mysql,
            db_port: None,
            db_ip: String::new(),
            db_user: None,
            db_password: None,
        };
        let mut ff = File::create("./config.yaml").unwrap();
        println!("{}", serde_yaml::to_string(&config).unwrap());
        match ff.write(serde_yaml::to_string(&config).unwrap().as_bytes()) {
            Err(why) => {
                if cfg!(feature = "log") {
                    error(format!("couldn't write to config : {}", why.to_string()));
                }
                exit(1)
            }
            Ok(_) => {
                if cfg!(feature = "log") {
                    info("successfully wrote to config")
                }
            }
        }
        config
    }
}
