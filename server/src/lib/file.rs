use actix_web::HttpRequest;
use shared::{JsonStruct, Folder, FType, JsonStructB};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

pub fn dir_content(req: &HttpRequest) -> String {
    let mut vec: Vec<String> = Vec::new();
    let path = crate::lib::http::without_cli(req.path());
    match fs::read_dir(path) {
        Ok(_f) => {
            for path in _f {
                vec.push(path.unwrap().path().display().to_string());
            }
        }
        Err(_e) => {
            vec.push(String::from("Error"));
            println!("Le dossier est inexistant");
        }
    };
    let mut folder = JsonStruct {
        result: true,
        lenght: vec.len() as i64,
        content: vec.to_owned(),
    };
    if vec.starts_with(&[String::from("Error")]) {
        folder.result = false;
    }
    match serde_json::to_string(&folder) {
        Ok(e) => e,
        Err(_e) => String::from("Not Work"),
    }
}



pub fn dir_contentb(req: &HttpRequest) -> String {
    let mut vec: Vec<Folder> = Vec::new();
    match fs::read_dir(crate::lib::http::without_cli(req.path())) {
        Ok(_f) => {
            for path in _f {
                match path {
                    Ok(f) => {
                        match f.metadata() {
                            Ok(e) => {
                                if e.is_file() == true {
                                    vec.push(Folder{
                                        result: true,
                                        name: f.file_name().to_str().unwrap().parse().unwrap(),
                                        ftype: FType::File
                                    });
                                } else {
                                    vec.push(Folder{
                                        result: true,
                                        name: f.file_name().to_str().unwrap().parse().unwrap(),
                                        ftype: FType::Folder
                                    });
                                }

                            }
                            Err(_e) => {

                            }
                        }
                    }
                    Err(_e) => {
                        vec.push(Folder{
                            result: false,
                            name: "Error".to_string(),
                            ftype: FType::Error
                        });
                    }
                }

            }
        }
        Err(_e) => {
            vec.push(Folder{
                result: false,
                name: "Error".to_string(),
                ftype: FType::Error
            });
            println!("Le dossier est inexistant");
        }
    };
    let mut folder = JsonStructB {
        result: true,
        lenght: vec.len() as i64,
        rtype: FType::Folder,
        content: vec
    };
   /* if vec.starts_with(&[String::from("Error")]) {
        folder.result = false;
    }*/
    match serde_json::to_string(&folder) {
        Ok(e) => e,
        Err(_e) => String::from("Not Work"),
    }
}
pub fn compress(folder: String, type_compress: &String) -> ZipResult<()> {
    let file = File::create(&folder).unwrap();
    let buf = PathBuf::from(&folder);
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&buf)
}
