use actix_web::HttpRequest;
use shared::{Folder, FType, JsonStruct};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

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
pub fn _compress(folder: String, _type_compress: &String) -> ZipResult<()> {
    let file = File::create(&folder).unwrap();
    let buf = PathBuf::from(&folder);
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&buf)
}
