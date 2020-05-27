use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;

use crate::lib::config::Config;

pub fn default() {

   let vec =  match read_dir(PathBuf::from("./")) {
        Ok(O) => {
            let mut vec : Vec<&str> = Vec::new();
            for epath in O{
                vec.push(epath.unwrap().file_name().to_str().unwrap());
            }
            vec
        },
        Err(_e) => {    let vec : Vec<&str> = Vec::new(); vec}
    };
    if vec.contains(&"config.toml") {
        File::create("config.toml");
        let config = Config {
            ip: "0.0.0.0".to_string(),
            port: 8081,
            folder_root: "/".to_string()
        };
        let file = File::open("config.toml");

        file.unwrap().write_all(serde_yaml::to_string(&config).unwrap().as_ref());
    }
}