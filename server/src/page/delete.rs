use crate::lib::http::http::without_api;
use actix_web::{delete, Error, HttpRequest, HttpResponse, web};
use shared::{FType, Folder, JsonStruct};
use crate::lib::db::user::valid_session::valid_session;

#[delete("/api/file/{path:.*}")]
pub async fn deletef(req: HttpRequest, path: web::Path<(String)>) -> Result<HttpResponse, Error> {
    let mut result = JsonStruct {
        result: false,
        lenght: 0,
        ftype: FType::File,
        content: vec![],
    };
    if let Some(e) = req.headers().get("token") {
        if valid_session(String::from(e.to_str().expect("Parse Str Error"))) {
            match std::fs::remove_dir_all(format!("/{}", path.0)) {
                Ok(_o) => {
                    result.result = true;
                    result.content = vec![Folder {
                        result: true,
                        name: "Work".to_string(),
                        ftype: "File".to_string(),
                    }]
                }
                Err(_e) => {
                    result.content = vec![Folder {
                        result: false,
                        name: "Error".to_string(),
                        ftype: "Error".to_string(),
                    }]
                }
            };
            Ok(HttpResponse::Ok()
                .header("charset", "utf-8")
                .header("Access-Control-Allow-Origin", "*")
                .body(serde_json::to_string(&result).unwrap()))
        } else {
            Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }
    } else {
        Ok(HttpResponse::Ok().body(String::from("No token provided")))
    }
}
