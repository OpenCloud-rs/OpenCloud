use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::lib::config::config::Config;
use std::process::exit;

pub fn default() -> Config {
    let mut vec: Vec<String> = Vec::new();
    let rd = read_dir(PathBuf::from("./")).expect("Error: Can't read the folder");
    for rde in rd {
        let de = rde.expect("Error: Can't read Dir Entry");
        vec.push(de.file_name().into_string().expect("Error: Bad name"));
    }

    if !vec.contains(&String::from("temp")) {
        std::fs::create_dir("./temp").expect("Error");
    }
    if !vec.contains(&String::from("db.sql")) {
        std::fs::File::create("./db.sql").expect("Error");
    }
    if !vec.contains(&String::from("home")) {
        std::fs::create_dir("./home").expect("Error");
    }
    if !vec.contains(&String::from("config.yaml")) {
        let config = Config {
            server_ip: "0.0.0.0".to_string(),
            server_port: 8081,
            folder_root: "/".to_string(),
        };
        let mut ff = File::create("./config.yaml").unwrap();
        match ff.write_all(serde_yaml::to_string(&config).unwrap().as_bytes()) {
            Err(why) => panic!("couldn't write to config : {}", why.to_string()),
            Ok(_) => println!("successfully wrote to config"),
        }
        config
    } else {
        let mut buf = String::new();
        File::open("./config.yaml")
            .expect("Can't found the config")
            .read_to_string(&mut buf)
            .expect("Can't read the config");
        match serde_yaml::from_str(&buf) {
            Ok(o) => o,
            Err(_e) => {
                println!("Config Error");
                exit(1);
            }
        }
    }
}
