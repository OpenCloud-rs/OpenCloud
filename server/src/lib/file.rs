use actix_files::file_extension_to_mime;
use actix_web::HttpRequest;
use shared::{FType, Folder, JsonStruct};
use std::fs;
use std::fs::{metadata, File};
use std::io::Read;
use std::path::PathBuf;
use zip_extensions::*;
use crate::lib::http::without_api;

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
                                            //println!("{} => {:?}",format!["{}{}", path, f.file_name().to_str().unwrap()].to_string(), file_extension_to_mime(format!["{}{}", path, f.file_name().to_str().unwrap()].to_string().as_ref()))
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
    /*if content.starts_with(&[Folder {result: false, name: "Error".to_string(), ftype: FType::Error }]) {
        folder.result = false;
    }*/
    match serde_json::to_string(&folder) {
        Ok(e) => e,
        Err(_e) => String::from("Not Work"),
    }
}

pub fn get_file_as_byte_vec(filename: String, compress: &str,) -> Vec<u8> {
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
                    "tar" => {
                        File::create("./folder.tar").unwrap();
                        tar::Builder::new(File::open("./folder.tar").expect("no file found")).append_dir_all("./folder.tar",without_api(filename.as_str()));
                        File::open("./folder.tar").expect("no file found")
                    }
                    _ => {
                        File::create("./folder.zip").unwrap();
                        println!("filename => {}", without_api(filename.as_ref()));
                        match zip_create_from_directory(&PathBuf::from("./folder.zip"), &PathBuf::from(without_api(filename.as_ref()))) {
                            Ok(n) => {
                                println!("Zip is Ok");
                            }
                            Err(e) => {
                                println!("Error : {}", e)
                            }
                        }
                        File::open("./folder.zip").expect("no file found")
                    }
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

pub fn get_mime(file: &str) -> String {
    mime_guess::from_path(file.clone())
        .first_or_octet_stream()
        .to_string()
}
