use crate::lib::archive::archive::random_archive;
use crate::lib::http::http::without_api;
use actix_files::file_extension_to_mime;
use actix_web::{HttpResponse as Response, Error};
use actix_web::body::Body;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use shared::{FType, Folder, JsonStruct};
use std::fs;
use std::fs::{metadata, read_dir};
use tokio::io::AsyncReadExt;
use actix_utils::mpsc;

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
    let root = if cfg!(windows) {
        "C:"
    } else {
        ""
    };

    match fs::metadata(format!("{}{}", root, path.clone())) {
        Ok(e) => {
            if e.is_file() == true {
                result = true;
                ftype = FType::File;
                content.push(Folder {
                    result: true,
                    size: e.len(),
                    created: time::PrimitiveDateTime::from(e.created().expect("Error")).format("%d-%m-%Y %T"),
                    name: String::from(path.split("/").last().unwrap()),
                    ftype: file_extension_to_mime(path.split("/").last().unwrap()).to_string(),
                    modified: time::PrimitiveDateTime::from(e.modified().expect("Error")).format("%d-%m-%Y %T"),
                });
            } else if e.is_dir() == true {
                match fs::read_dir(path.clone()) {
                    Ok(e) => {
                        result = true;
                        ftype = FType::Folder;
                        for dpath in e {
                            match dpath {
                                Ok(f) => {
                                    match f.metadata() {
                                        Ok(e) => {
                                            if e.is_file() == true {
                                                content.push(Folder {
                                                    result: true,
                                                    name: String::from(
                                                        f.file_name().to_str().unwrap_or("Bad Name"),
                                                    ),
                                                    ftype: get_mime(
                                                        f.file_name().to_str().unwrap_or("Bad Type"),
                                                    ),
                                                    size: e.len(),
                                                    created: time::PrimitiveDateTime::from(f.metadata().expect("Error").created().expect("Error")).format("%d-%m-%Y %T"),
                                                    modified: time::PrimitiveDateTime::from(f.metadata().expect("Error").modified().expect("Error")).format("%d-%m-%Y %T"),
                                                });
                                            } else {
                                                content.push(Folder {
                                                    result: true,
                                                    name: String::from(
                                                        f.file_name().to_str().unwrap_or("Bad File Type"),
                                                    ),
                                                    size: get_size_dir(format!("{}{}/{}", root, path.clone(), f.file_name().to_str().unwrap_or("Bad File Type"))),
                                                    created: time::PrimitiveDateTime::from(f.metadata().expect("Error").created().expect("Error")).format("%d-%m-%Y %T"),
                                                    modified: time::PrimitiveDateTime::from(f.metadata().expect("Error").modified().expect("Error")).format("%d-%m-%Y %T"),
                                                    ftype: String::from("Folder"),
                                                });
                                            }
                                        }
                                        Err(_e) => content.push(Folder {
                                            result: false,
                                            size: 0,
                                            created: String::from("0-0-0000 00:00:00"),
                                            modified: String::from("0-0-0000 00:00:00"),
                                            name: String::from("Error"),
                                            ftype: String::from("Error"),

                                        }),
                                    }
                                }
                                Err(_e) => {
                                    content.push(Folder {
                                        result: false,
                                        size: 0,
                                        created: String::from("0-0-0000 00:00:00"),
                                        modified: String::from("0-0-0000 00:00:00"),
                                        name: String::from("Error"),
                                        ftype: String::from("Error"),

                                    });
                                }
                            }
                        }
                    }
                    Err(_e) => {
                        content.push(Folder {
                            result: false,
                            size: 0,
                            created: String::from("0-0-0000 00:00:00"),
                            name: String::from("Folder Not Work"),
                            ftype: String::from("Error"),
                            modified: String::from("0-0-0000 00:00:00"),
                        });
                        println!("Le dossier est inexistant");
                    }
                }
            }
        }
        Err(_e) => {}
    }
    match sort {
        Sort::Name => { content.sort_by(|a, b| a.name.cmp(&b.name)); }
        Sort::Type => { content.sort_by(|a, b| b.ftype.cmp(&a.ftype)); }
        Sort::Size => { content.sort_by(|a, b| b.size.cmp(&a.size)); }
        Sort::Date => { content.sort_by(|a, b| b.created.cmp(&a.created)); }
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

pub async fn get_file_as_byte_vec(mut filename: String, compress: &str) -> Vec<u8> {
    filename = without_api(filename.as_str()).to_string();
    println!("{}", filename);
    match metadata(filename.clone()) {
        Ok(e) => {
            if e.is_file() {
                let mut buf: Vec<u8> = vec![0; e.len() as usize];
                match tokio::fs::File::open(filename.clone()).await {
                    Ok(mut o) => {
                        o.read(&mut buf).await.expect("Error");
                    }
                    Err(e) => {
                        println!("{} => {}", filename.clone(), e);
                    }
                }
                buf
            } else if e.is_dir() {
                let mut file = match compress.to_lowercase().as_str() {
                    "tar" => random_archive("tar.gz".to_string(), filename),
                    _ => random_archive("zip".to_string(), filename),
                }
                    .await;
                println!("{}", file.metadata().await.unwrap().len());

                let mut buf: Vec<u8> = vec![0; file.metadata().await.unwrap().len() as usize];
                match file.read_to_end(&mut buf).await {
                    Ok(e) => {
                        println!("{}", e);
                    }
                    Err(e) => println!("{:?}", e),
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
        .body(crate::lib::file::file::dir_content(path, sort)))
}

pub fn get_size_dir(path: String) -> u64 {
    let mut size: u64 = 0;
    match read_dir(path) {
        Ok(e) => {
            for entry in e {
                let sizef = entry.expect("Error").metadata().expect("Error").len();
                size += sizef;
            }
        }
        Err(_e) => {}
    }
    size
}

use bytes::Bytes;
use actix_http::Payload;
use futures::StreamExt;

pub async fn get_file_preview(path: String) -> std::io::Result<Response<Body>> {
    let (mut tx, rx_body) = mpsc::channel();

    let mut try_file = tokio::fs::File::open(path.clone()).await;
    if try_file.is_err() {
        return Ok(Response::Ok().header("Access-Control-Allow-Origin", "*")
            .header("charset", "utf-8").body("Error"))
    }
    
    let mut buf: Vec<u8> = Vec::new();
    match try_file.expect("Error").read_to_end(&mut buf).await {
        Ok(e) => {
            println!("{}", e);
        }
        Err(e) => println!("{:?}", e),
    };

    let _ = tx.send(Ok::<_, Error>(actix_web::web::Bytes::from(buf.clone())));

    Ok(Response::Ok().header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8").content_type(get_mime(
        path.clone().as_str(),
    )).streaming(rx_body))
}