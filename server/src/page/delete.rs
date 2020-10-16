use actix_web::{delete, Error, HttpRequest, HttpResponse, web};
use shared::{FType, Folder, JsonStruct};
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::db::log::insert::insert;
use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::log::model::ActionType;

#[delete("/api/file/{path:.*}")]
pub async fn deletef(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, Error> {
    let mut result = JsonStruct {
        result: false,
        lenght: 0,
        ftype: FType::File,
        content: Vec::new(),
    };
    if let Some(e) = req.headers().get("token") {
        if valid_session(String::from(e.to_str().expect("Parse Str Error"))) {

            match std::fs::remove_dir_all(format!("/{}", path.0)) {
                Ok(_o) => {
                    result.result = true;
                    result.content.push(Folder {
                        result: true,
                        size: 0,
                        created: String::from("0-0-0000 00:00:00"),
                        name: "Work".to_string(),
                        ftype: "File".to_string(),
                        modified: String::from("0-0-0000 00:00:00")
                    });
                    let user = get_user_by_token(String::from(e.to_str().expect("Parse Str Error"))).unwrap();
                    tokio::spawn(async move {
                        insert(user.id, ActionType::Delete)
                    }).await.expect("Error");
                }
                Err(_e) => {
                    result.content.push(Folder {
                        result: false,
                        size: 0,
                        created: String::from("0-0-0000 00:00:00"),
                        name: "Error".to_string(),
                        ftype: "Error".to_string(),
                        modified: String::from("0-0-0000 00:00:00")
                    })
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
