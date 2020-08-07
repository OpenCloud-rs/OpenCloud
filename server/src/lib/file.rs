use crate::lib::http::without_api;
use actix_files::file_extension_to_mime;
use actix_web::HttpRequest;
use shared::{FType, Folder, JsonStruct};
use std::fs;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::PathBuf;
use zip_extensions::*;

pub fn dir_content(req: &HttpRequest) -> String {
    let path = without_api(req.path());

    let mut content: Vec<Folder> = Vec::new();
    let mut result: bool = false;
    let mut ftype: FType = FType::Error;

    match fs::metadata(path) {
        Ok(e) => {
            if e.is_file() == true {
                result = true;
                ftype = FType::File;
                content.push(Folder {
                    result: true,
                    name: String::from(path.split("/").last().unwrap()),
                    ftype: file_extension_to_mime(path.split("/").last().unwrap()).to_string(),
                });
            } else if e.is_dir() == true {
                match fs::read_dir(path) {
                    Ok(e) => {
                        for dpath in e {
                            match dpath {
                                Ok(f) => {
                                    result = true;
                                    ftype = FType::Folder;
                                    match f.metadata() {
                                        Ok(e) => {
                                            if e.is_file() == true {
                                                content.push(Folder {
                                                    result: true,
                                                    name: f
                                                        .file_name()
                                                        .to_str()
                                                        .unwrap()
                                                        .parse()
                                                        .unwrap(),
                                                    ftype: get_mime(
                                                        f.file_name().to_str().unwrap(),
                                                    ),
                                                });
                                            } else {
                                                content.push(Folder {
                                                    result: true,
                                                    name: f
                                                        .file_name()
                                                        .to_str()
                                                        .unwrap()
                                                        .parse()
                                                        .unwrap(),
                                                    ftype: String::from("Folder"),
                                                });
                                            }
                                        }
                                        Err(_e) => content.push(Folder {
                                            result: false,
                                            name: "Error".to_string(),
                                            ftype: String::from("Error"),
                                        }),
                                    }
                                }
                                Err(_e) => {
                                    content.push(Folder {
                                        result: false,
                                        name: "Error".to_string(),
                                        ftype: String::from("Error"),
                                    });
                                }
                            }
                        }
                    }
                    Err(_e) => {
                        content.push(Folder {
                            result: false,
                            name: "Folder Not Work".to_string(),
                            ftype: String::from("Error"),
                        });
                        println!("Le dossier est inexistant");
                    }
                }
            }
        }
        Err(_e) => {}
    }
    let folder = JsonStruct {
        result,
        lenght: content.len() as i64,
        ftype,
        content,
    };
    match serde_json::to_string(&folder) {
        Ok(e) => e,
        Err(_e) => String::from("Not Work"),
    }
}

pub fn get_file_as_byte_vec(filename: String, compress: &str) -> Vec<u8> {
    match metadata(without_api(filename.as_ref())) {
        Ok(e) => {
            if e.is_file() {
                let mut buf: Vec<u8> = vec![0; e.len() as usize];
                File::open(filename)
                    .expect("no file found")
                    .read(&mut buf)
                    .expect("Buffer overflow");
                buf
            } else if e.is_dir() {
                let mut file = match compress.to_lowercase().as_str() {
                    "tar" => random_archive("tar.gz".to_string(), filename),
                    _ => random_archive("zip".to_string(), filename),
                };
                let mut buf: Vec<u8> = vec![0; file.metadata().unwrap().len() as usize];
                file.read(&mut buf).expect("Buffer overflow");
                buf
            } else {
                let buf: Vec<u8> = String::from("Error").as_bytes().to_vec();
                buf
            }
        }
        Err(_e) => {
            let buf: Vec<u8> = String::from("Error").as_bytes().to_vec();
            buf
        }
    }
}

fn tar_archive(name: String, dir: String) -> File {
    let file_name = format!("./temp/{}.tar.gz", name);
    File::create(&file_name).unwrap();
    tar::Builder::new(File::open(&file_name).expect("no file found"))
        .append_dir_all(&file_name, dir.as_str()).expect("Error");
    File::open(&file_name).expect("no file found")
}

fn zip_archive(name: String, dir: String) -> File {
    let file_name = format!("./temp/{}.zip", name);
    File::create(&file_name).unwrap();
    println!("filename => {}", dir);
    match zip_create_from_directory(&PathBuf::from(&file_name), &PathBuf::from(dir)) {
        Ok(_n) => {
            println!("Zip is Ok");
        }
        Err(e) => println!("Error : {}", e),
    }
    File::open(file_name).expect("no file found")
}

fn random_archive(extention: String, dir: String) -> File {
    let name: String = random_name();
    let dir: &str = without_api(dir.as_ref());
    if extention == String::from("tar.gz") {
        tar_archive(name, dir.to_string())
    } else {
        zip_archive(name, dir.to_string())
    }
}

fn random_name() -> String {
    use rand::Rng;
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ
    ";
    let mut rng = rand::thread_rng();
    (0..30)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn get_mime(file: &str) -> String {
    mime_guess::from_path(file.clone())
        .first_or_octet_stream()
        .to_string()
}
