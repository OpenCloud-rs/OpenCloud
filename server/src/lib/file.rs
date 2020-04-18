
    use std::fs;
    use shared::Folder;
    use actix_web::HttpRequest;

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
    let mut folder = Folder {
        result: true,
        lenght: vec.len() as i64,
        content: vec.to_owned(),
    };
    if vec.starts_with(&[String::from("Error")]) {
        folder.result = false;
    }
    match serde_json::to_string(&folder) {
        Ok(e) => {
            e
        },
        Err(_e) => {
            String::from("Not Work")
        }
    }

}
