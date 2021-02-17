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
                                        content.push(Folder::from_metadata(
                                            e.clone(),
                                            format!(
                                                "{}{}",
                                                path,
                                                f.file_name().to_str().unwrap_or("Bad Name")
                                            ),
                                        ));
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
    let mut buf: Vec<u8> = Vec::new();
    if let Ok(e) = metadata(filename.clone()) {
        if e.is_file() {
            if let Ok(mut file) = async_std::fs::File::open(filename.clone()).await {
                if let Ok(_) = file.read(&mut buf).await {}
            }
        } else {
            let mut file = match compress.to_lowercase().as_str() {
                "tar" => random_archive("tar.gz".to_string(), filename),
                _ => random_archive("zip".to_string(), filename),
            }
            .await;
            if cfg!(debug_assertions) {
                println!("{}", file.metadata().await.unwrap().len());
            }
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
        }
    }
    if buf.is_empty() {
        let vec: Vec<u8> = String::from("Error").as_bytes().to_vec();
        buf = vec;
    }
    buf
}

pub fn get_dir(path: String, sort: Sort) -> Response<Body> {
    Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(crate::lib::file::dir_content(path, sort))
}

pub fn get_size_dir(path: String) -> u64 {
    let mut size: u64 = 0;
    if let Ok(readdir) = read_dir(path) {
        for i in readdir {
            if let Ok(dentry) = i {
                if let Ok(e) = dentry.metadata() {
                    size += e.len()
                }
            }
        }
    }

    size
}

pub async fn get_file_preview(path: String) -> Response<Body> {
    let (tx, rx_body) = mpsc::channel();

    let try_file = async_std::fs::File::open(path.clone()).await;
    if try_file.is_err() {
        return Response::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .header("charset", "utf-8")
            .body("Error");
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

    Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type(
            mime_guess::from_ext(path.split("/").last().unwrap())
                .first_or_octet_stream()
                .to_string(),
        )
        .streaming(rx_body)
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
        let ftype;
        let size;
        if e.is_dir() {
            ftype = "Folder".to_string();
            size = get_size_dir(path.clone())
        } else {
            ftype = mime_guess::from_path(path.split("/").last().unwrap())
                .first_or_octet_stream()
                .to_string();
            size = e.len()
        };
        Folder {
            result: true,
            size,
            created: time::PrimitiveDateTime::from(
                e.created().unwrap_or(std::time::SystemTime::now()),
            )
            .format("%d-%m-%Y %T"),
            name: String::from(path.trim_end_matches("/").split("/").last().unwrap()),
            ftype,
            modified: time::PrimitiveDateTime::from(
                e.modified().unwrap_or(std::time::SystemTime::now()),
            )
            .format("%d-%m-%Y %T"),
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
