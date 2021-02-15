pub mod default;

use crate::lib::archive::random_archive;
use actix_utils::mpsc;
use actix_web::body::Body;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{Error, HttpResponse as Response};
use async_std::io::ReadExt;
use fs::Metadata;
use logger::{error, warn};
use shared::{FType, Folder, JsonStruct};
use std::fs;
use std::fs::{metadata, read_dir};

pub enum Sort {
    Name,
    Type,
    Size,
    Date,
}

pub fn dir_content(path: String, sort: Sort) -> String {
    let mut content: Vec<Folder> = Vec::new();
    let mut result: bool = false;
    let mut ftype: FType = FType::Error;

    let root = if cfg!(windows) { "C:" } else { "" };
    if !inhome(path.clone()) {
        return String::from("Stay at home please");
    }
    match fs::metadata(format!("{}{}", root, path.clone())) {
        Ok(e) => {
            if e.is_file() == true {
                result = true;
                ftype = FType::File;
                content.push(Folder::from_metadata(e.clone(), path.clone()));
            } else if e.is_dir() == true {
                match fs::read_dir(path.clone()) {
                    Ok(e) => {
                        result = true;
                        ftype = FType::Folder;
                        for dpath in e {
                            match dpath {
                                Ok(f) => match f.metadata() {
                                    Ok(e) => {
                                        if e.is_file() == true {
                                            content.push(Folder::from_metadata(
                                                e.clone(),
                                                f.file_name().to_str().unwrap_or("Bad Name").to_string(),
                                            ));
                                        } else {
                                            content.push(Folder::from_metadata(
                                                e.clone(),
                                                f.file_name().to_str().unwrap_or("Bad Name").to_string(),
                                            ));
                                        }
                                    }
                                    Err(_) => content.push(Folder::error("Error".to_string())),
                                },
                                Err(_) => {
                                    content.push(Folder::error("Error".to_string()));
                                }
                            }
                        }
                    }
                    Err(_) => {
                        content.push(Folder::error("Folder not work".to_string()));
                        if cfg!(feature = "log") {
                            warn("Le dossier est inexistant".to_string());
                        }
                    }
                }
            }
        }
        Err(_) => {
            content.push(Folder::error("Error".to_string()));
        }
    }

    match sort {
        Sort::Name => {
            content.sort_by(|a, b| a.name.cmp(&b.name));
        }
        Sort::Type => {
            content.sort_by(|a, b| b.ftype.cmp(&a.ftype));
        }
        Sort::Size => {
            content.sort_by(|a, b| b.size.cmp(&a.size));
        }
        Sort::Date => {
            content.sort_by(|a, b| b.created.cmp(&a.created));
        }
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

pub async fn get_file_as_byte_vec(filename: String, compress: &str) -> Vec<u8> {
    match metadata(filename.clone()) {
        Ok(e) => {
            if e.is_file() {
                let mut buf: Vec<u8> = Vec::new();
                match async_std::fs::File::open(filename.clone()).await {
                    Ok(mut o) => {
                        if let Ok(_) = o.read(&mut buf).await {
                        } else {
                            if cfg!(feature = "log") {
                                error("Read Error".to_string())
                            }
                        };
                    }
                    Err(_) => {
                        if cfg!(feature = "log") {
                            error(format!("Error : Can't Opening file"));
                        }
                    }
                }
                buf
            } else if e.is_dir() {
                let mut file = match compress.to_lowercase().as_str() {
                    "tar" => random_archive("tar.gz".to_string(), filename),
                    _ => random_archive("zip".to_string(), filename),
                }
                .await;
                if cfg!(debug_assertions) {
                    println!("{}", file.metadata().await.unwrap().len());
                }
                let mut buf: Vec<u8> = Vec::new();
                match file.read_to_end(&mut buf).await {
                    Ok(e) => {
                        if cfg!(debug_assertions) {
                            println!("{}", e);
                        }
                    }
                    Err(e) => {
                        if cfg!(feature = "log") {
                            error(format!("{:?}", e))
                        }
                    }
                };
                buf
            } else {
                let buf: Vec<u8> = String::from("Error").as_bytes().to_vec();
                buf
            }
        }
        Err(e) => {
            println!("{:?}", e);
            let buf: Vec<u8> = String::from("Error").as_bytes().to_vec();
            buf
        }
    }
}

pub fn get_mime(file: &str) -> String {
    mime_guess::from_path(file)
        .first_or_octet_stream()
        .to_string()
}

pub fn get_dir(path: String, sort: Sort) -> std::io::Result<Response<Body>> {
    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(crate::lib::file::dir_content(path, sort)))
}

pub fn get_size_dir(path: String) -> u64 {
    let mut size: u64 = 0;
    match read_dir(path) {
        Ok(e) => {
            for entry in e {
                match entry {
                    Ok(dentry) => match dentry.metadata() {
                        Ok(e) => {
                            size += e.len();
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
        }
        Err(_e) => {}
    }
    size
}

pub async fn get_file_preview(path: String) -> std::io::Result<Response<Body>> {
    let (tx, rx_body) = mpsc::channel();

    let try_file = async_std::fs::File::open(path.clone()).await;
    if try_file.is_err() {
        return Ok(Response::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .header("charset", "utf-8")
            .body("Error"));
    }

    let mut buf: Vec<u8> = Vec::new();
    if let Ok(mut f) = try_file {
        match f.read_to_end(&mut buf).await {
            Ok(e) => {
                if cfg!(debug_assertions) {
                    println!("{}", e);
                }
            }
            Err(e) => error(format!("{:?}", e)),
        };
    }

    let _ = tx.send(Ok::<_, Error>(actix_web::web::Bytes::from(buf.clone())));

    Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type(get_mime(path.clone().as_str()))
        .streaming(rx_body))
}

pub fn inhome(path: String) -> bool {
    let split: Vec<&str> = path.split("/").collect();
    let mut n = 0;
    // let clean_path = path.replace("/..", "");
    for a in split.clone() {
        if a == ".." {
            n += 1;
        };
    }
    let mut result = String::new();
    let mut e = 0;
    for a in split.clone() {
        if e == n && n != 0 {
            break;
        } else {
            result.push_str(format!("{}/", a).as_str());
        }
        e += 1;
    }
    if result.contains(format!("./home/{}", split[2]).as_str()) {
        true
    } else {
        false
    }
}

trait TraitFolder {
    fn from_metadata(e: Metadata, path: String) -> Folder;
    fn error(error: String) -> Folder;
}

impl TraitFolder for Folder {
    fn from_metadata(e: Metadata, path: String) -> Folder {
        println!("{}", path);
        if e.is_dir() {
            Folder {
                result: true,
                size: get_size_dir(path.clone()),
                created: time::PrimitiveDateTime::from(
                    e.created().unwrap_or(std::time::SystemTime::now()),
                )
                .format("%d-%m-%Y %T"),
                name: String::from(path.trim_end_matches("/").split("/").last().unwrap()),
                ftype: mime_guess::from_ext(path.split("/").last().unwrap())
                    .first_or_octet_stream()
                    .to_string(),
                modified: time::PrimitiveDateTime::from(
                    e.modified().unwrap_or(std::time::SystemTime::now()),
                )
                .format("%d-%m-%Y %T"),
            }
        } else {
            Folder {
                result: true,
                size: e.len(),
                created: time::PrimitiveDateTime::from(
                    e.created().unwrap_or(std::time::SystemTime::now()),
                )
                .format("%d-%m-%Y %T"),
                name: String::from(path.trim_end_matches("/").split("/").last().unwrap()),
                ftype: mime_guess::from_ext(path.split("/").last().unwrap())
                    .first_or_octet_stream()
                    .to_string(),
                modified: time::PrimitiveDateTime::from(
                    e.modified().unwrap_or(std::time::SystemTime::now()),
                )
                .format("%d-%m-%Y %T"),
            }
        }
    }
    fn error(error: String) -> Folder {
        Folder {
            result: false,
            size: 0,
            created: String::from("0-0-0000 00:00:00"),
            modified: String::from("0-0-0000 00:00:00"),
            name: String::from(error),
            ftype: String::from("Error"),
        }
    }
}
