use actix_web::HttpRequest;
use shared::{FolderB, FType, JsonStruct, JsonStructB};
use std::fs;
use std::path::PathBuf;
use zip_extensions::*;
use actix_files::file_extension_to_mime;
use crate::lib::http::without_cli;
use std::fs::File;
use std::io::Read;

pub fn dir_content(req: &HttpRequest) -> String {
    let path = without_cli(req.path());
    let mut content: Vec<FolderB> = Vec::new();
    let mut result: bool = false;
    let mut ftype: FType = FType::Error;
    match fs::metadata(crate::lib::http::without_cli(req.path())) {
        Ok(e) => {
            if e.is_file() == true {
                result = true;
                ftype = FType::File;
                content.push(FolderB{
                    result: true,
                    name: String::from(path.split("/").last().unwrap()),
                    ftype: file_extension_to_mime(path.split("/").last().unwrap()).to_string()
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
                                                content.push(FolderB{
                                                    result: true,
                                                    name: f.file_name().to_str().unwrap().parse().unwrap(),
                                                    ftype: file_extension_to_mime(f.file_name().to_str().unwrap()).to_string()
                                                });
                                                //println!("{} => {:?}",format!["{}{}", path, f.file_name().to_str().unwrap()].to_string(), file_extension_to_mime(format!["{}{}", path, f.file_name().to_str().unwrap()].to_string().as_ref()))
                                            } else {
                                                content.push(FolderB{
                                                    result: true,
                                                    name: f.file_name().to_str().unwrap().parse().unwrap(),
                                                    ftype: String::from("Folder")
                                                });
                                            }

                                        }
                                        Err(_e) => {
                                            content.push(
                                                FolderB{
                                                    result: false,
                                                    name: "Error".to_string(),
                                                    ftype: String::from("Error")
                                                }
                                            )
                                        }
                                    }
                                }
                                Err(_e) => {
                                    content.push(FolderB{
                                        result: false,
                                        name: "Error".to_string(),
                                        ftype: String::from("Error")
                                    });
                                }
                            }
                        }
                    }
                    Err(_e) => {
                        content.push(FolderB{
                            result: false,
                            name: "Folder Not Work".to_string(),
                            ftype: String::from("Error")
                        });
                        println!("Le dossier est inexistant");
                    }
                }
            }
        }
        Err(_e) => {

        }
    }
    let folder = JsonStructB {
        result,
        lenght: content.len() as i64,
        ftype,
        content
    };
   /*if content.starts_with(&[FolderB {result: false, name: "Error".to_string(), ftype: FType::Error }]) {
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

pub fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
