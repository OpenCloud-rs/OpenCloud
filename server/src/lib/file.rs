use actix_web::HttpRequest;
use shared::{Folder, FType, JsonStruct};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use zip::*;
use zip_extensions::*;
use tar::Builder;

pub fn dir_content(req: &HttpRequest) -> String {
    let mut content: Vec<Folder> = Vec::new();
    let mut result: bool = false;
    let mut ftype: FType = FType::Error;
    match fs::metadata(crate::lib::http::without_cli(req.path())) {
        Ok(e) => {
            if e.is_file() == true {
                result = true;
                ftype = FType::File;
                content.push(Folder{
                    result: true,
                    name: String::from(crate::lib::http::without_cli(req.path()).split("/").last().unwrap()),
                    ftype: FType::File
                });
            } else if e.is_dir() == true {
                match fs::read_dir(crate::lib::http::without_cli(req.path())) {
                    Ok(e) => {
                        for path in e {
                            match path {
                                Ok(f) => {
                                    result = true;
                                    ftype = FType::Folder;
                                    match f.metadata() {
                                        Ok(e) => {
                                            if e.is_file() == true {
                                                content.push(Folder{
                                                    result: true,
                                                    name: f.file_name().to_str().unwrap().parse().unwrap(),
                                                    ftype: FType::File
                                                });
                                            } else {
                                                content.push(Folder{
                                                    result: true,
                                                    name: f.file_name().to_str().unwrap().parse().unwrap(),
                                                    ftype: FType::Folder
                                                });
                                            }

                                        }
                                        Err(_e) => {
                                            content.push(
                                                Folder{
                                                    result: false,
                                                    name: "Error".to_string(),
                                                    ftype: FType::Error
                                                }
                                            )
                                        }
                                    }
                                }
                                Err(_e) => {
                                    content.push(Folder{
                                        result: false,
                                        name: "Error".to_string(),
                                        ftype: FType::Error
                                    });
                                }
                            }
                        }
                    }
                    Err(_e) => {
                        content.push(Folder{
                            result: false,
                            name: "Folder Not Work".to_string(),
                            ftype: FType::Error
                        });
                        println!("Le dossier est inexistant");
                    }
                }
            }
        }
        Err(_e) => {

        }
    }
    let folder = JsonStruct {
        result,
        lenght: content.len() as i64,
        ftype,
        content
    };
   /*if content.starts_with(&[Folder {result: false, name: "Error".to_string(), ftype: FType::Error }]) {
        folder.result = false;
    }*/
    match serde_json::to_string(&folder) {
        Ok(e) => e,
        Err(_e) => String::from("Not Work"),
    }
}

pub fn compress(folder: String, type_compress: String) {
    let dd : String = String::from("tar");
    match type_compress {
        dd => {

        }
        _ => {
            let archive_file: PathBuf = PathBuf::from(&folder);
            let source_dir: PathBuf = PathBuf::from(&folder);
            zip_create_from_directory(&archive_file, &source_dir).unwrap()
        }
    }
}
