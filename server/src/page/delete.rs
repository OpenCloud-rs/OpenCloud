use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::{db::log::insert::insert, http::http::get_args};
use actix_web::{delete, web, Error, HttpRequest, HttpResponse};
use shared::{FType, Folder, JsonStruct};

#[delete("/file/{path:.*}")]
pub async fn deletef(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, Error> {
    let mut result = JsonStruct {
        result: false,
        lenght: 0,
        ftype: FType::File,
        content: Vec::new(),
    };
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or(""))
    } else if let Some(e) = get_args(req).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    if !e.is_empty() {
        if valid_session(String::from(e.clone())).await {
            let user = get_user_by_token(e.clone()).await.unwrap();
            println!("./home/{}/{}", user.name, path.0);
            if async_std::fs::metadata(format!("./home/{}/{}", user.name, path.0))
                .await
                .unwrap()
                .is_dir()
            {
                match async_std::fs::remove_dir_all(format!("./home/{}/{}", user.name, path.0))
                    .await
                {
                    Ok(_) => {
                        result.result = true;
                        result.content.push(Folder {
                            result: true,
                            size: 0,
                            created: String::from("0-0-0000 00:00:00"),
                            name: "Work".to_string(),
                            ftype: "File".to_string(),
                            modified: String::from("0-0-0000 00:00:00"),
                        });
                        insert(user.id, ActionType::Delete).await;
                    }
                    Err(e) => result.content.push(Folder {
                        result: false,
                        size: 0,
                        created: String::from("0-0-0000 00:00:00"),
                        name: e.to_string(),
                        ftype: "Error".to_string(),
                        modified: String::from("0-0-0000 00:00:00"),
                    }),
                };
            } else {
                match async_std::fs::remove_file(format!("./home/{}/{}", user.name, path.0)).await {
                    Ok(_) => {
                        result.result = true;
                        result.content.push(Folder {
                            result: true,
                            size: 0,
                            created: String::from("0-0-0000 00:00:00"),
                            name: "Work".to_string(),
                            ftype: "File".to_string(),
                            modified: String::from("0-0-0000 00:00:00"),
                        });
                        insert(user.id, ActionType::Delete).await;
                    }
                    Err(e) => result.content.push(Folder {
                        result: false,
                        size: 0,
                        created: String::from("0-0-0000 00:00:00"),
                        name: e.to_string(),
                        ftype: "Error".to_string(),
                        modified: String::from("0-0-0000 00:00:00"),
                    }),
                };
            }
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
