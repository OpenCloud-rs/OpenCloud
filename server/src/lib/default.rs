 use std::fs::{File, read_dir};
use std::io::{Write, Read};
use std::path::PathBuf;

use crate::lib::config::Config;
 use std::process::exit;

 pub fn default() -> Config{
    let mut vec : Vec<String> = Vec::new();
    match read_dir(PathBuf::from("./")) {
        Ok(O) => {
            for epath in O{
                match epath {
                    Ok(e) => {
                        match e.file_name().into_string() {
                            Ok(e) => {
                                vec.push(e)
                            }
                            _ => {}
                        }

                    },
                    Err(_e) => {}
                }
            }
        }
        Err(_e) => {}
    };
    if !vec.contains(&String::from("config.yaml")) {
        let config = Config {
            ip: "0.0.0.0".to_string(),
            port: 8081,
            folder_root: "/".to_string()
        };
        let mut ff = File::create("./config.yaml").unwrap();
        match ff.write_all(serde_yaml::to_string(&config).unwrap().as_bytes()) {
            Err(why) => panic!("couldn't write to config : {}",why.to_string()),
            Ok(_) => println!("successfully wrote to config"),
        }
        config
    } else {
        let mut buf = String::new();
        File::open("./config.yaml").unwrap().read_to_string(&mut buf).unwrap();
        match serde_yaml::from_str(&buf) {
            Ok(O) => {
                O
            }
            Err(_e) => {
                println!("Config Error");
                exit(1);
            }
        }
        /*let result : Config = serde_yaml::from_str(&buf).unwrap();
        result*/
    }
}
