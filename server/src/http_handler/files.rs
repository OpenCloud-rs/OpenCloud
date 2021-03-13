use actix_web::{get, delete, web, HttpRequest, HttpResponse};
use datagn::DatabasePool;
use crate::lib::db::log::insert::insert;
use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::file::{get_dir, get_file_preview, Sort};
use crate::lib::{archive::*, http::get_args};
use shared::{FType, Folder, JsonStruct};

#[get("/file/{path:.*}")]
pub async fn get_files(
    req: HttpRequest,
    path: web::Path<String>,
    data: web::Data<DatabasePool>,
) -> HttpResponse {
    let result;
    let mut database = data.get_ref().clone();

    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or_default())
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };

    if e.is_empty() {
        result = HttpResponse::BadRequest().body(String::from("No token provided"));
    } else if valid_session(&mut database, e.clone()).await {
        let bvec = get_args(req.clone());
        let user = match get_user_by_token(&mut database, e.clone()).await {
            Some(e) => e,
            None => {
                return HttpResponse::BadRequest().body(String::from("Error on get user"));
            }
        };
        if bvec.contains_key("download") {
            match bvec.get("download").unwrap_or(&String::new()).as_ref() {
                "tar.gz" | "tar" => {
                    result = download(
                        format!("{}/{}", user.home, path.0.clone()),
                        ArchiveType::Targz,
                    )
                    .await;
                }
                _ => {
                    result = download(
                        format!("{}/{}", user.home, path.0.clone()),
                        ArchiveType::Zip,
                    )
                    .await;
                }
            }
        } else if bvec.contains_key("sort") {
            match bvec.get("sort").unwrap_or(&String::new()).as_ref() {
                "by_size" => {
                    result = get_dir(format!("{}/{}", user.home, path.0.clone()), Sort::Size);
                }
                "by_name" => {
                    result = get_dir(format!("{}/{}", user.home, path.0.clone()), Sort::Name);
                }
                "by_date" => {
                    result = get_dir(format!("{}/{}", user.home, path.0.clone()), Sort::Date);
                }
                _ => {
                    result = get_dir(format!("{}/{}", user.home, path.0.clone()), Sort::Type);
                }
            }
        } else if bvec.contains_key("preview") {
            result = get_file_preview(format!("{}/{}", user.home, path.0.clone())).await
        } else {
            result = get_dir(format!("{}/{}", user.home, path.0.clone()), Sort::Name);
        }
        insert(&mut database, user.id, ActionType::Get).await;
    } else {
        result = HttpResponse::BadRequest().body("The token provided isn't valid")
    }

    result
}

use actix_multipart::Multipart;
use actix_web::{post, Error};
use async_std::io::prelude::WriteExt;
use tokio_stream::StreamExt;

#[post("/file/{path:.*}")]
pub async fn save_file(
    req: HttpRequest,
    mut payload: Multipart,
    path: web::Path<String>,
    data: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or(""))
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    let mut database = data.get_ref().clone();
    if e.is_empty() {
        Ok(HttpResponse::BadRequest().body("No token provided"))
    } else {
        let url = format!("/{}", path.0);
        if valid_session(&mut database, e.clone()).await {
            let user = match get_user_by_token(&mut database, e.clone()).await {
                Some(e) => e,
                None => {
                    return Ok(HttpResponse::Ok().body("Can't get user"));
                }
            };
            insert(&mut database, user.id, ActionType::Upload).await;
            let mut result = false;
            while let Ok(Some(mut field)) = payload.try_next().await {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(ToString::to_string))
                    .expect("Can't get field name!");
                let filepath = format!(
                    "./home/{}/{}/{}",
                    user.name,
                    url.strip_prefix("/").unwrap(),
                    filename
                );

                if cfg!(debug_assertions) {
                    println!(
                    "--------------------- Url : {}, Name: {}, Path: {} ---------------------------",
                    url, filename, filepath
                );
                }
                let mut f = async_std::fs::File::create(filepath.clone()).await.unwrap();

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(e) => {
                            f = f.write_all(&e).await.map(|_| f).unwrap();
                        }
                        Err(e) => {
                            if cfg!(features = "log") {
                                logger::error(format!("{:?}", e));
                            }
                        }
                    }
                }
                result = true;
            }
            if result {
                return Ok(HttpResponse::Ok().body("The file is uploaded"));
            } else {
                return Ok(HttpResponse::BadRequest().body("Error on uploading the file"));
            }
        } else {
            Ok(HttpResponse::BadRequest().body("The token provided isn't valid"))
        }
    }
}

#[delete("/file/{path:.*}")]
pub async fn delete_file(
    req: HttpRequest,
    path: web::Path<String>,
    data: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut result = JsonStruct {
        result: false,
        lenght: 0,
        ftype: FType::File,
        content: Vec::new(),
    };
    let mut database = data.get_ref().clone();
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or(""))
    } else if let Some(e) = get_args(req).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    if e.is_empty() {
        Ok(HttpResponse::Ok().body(String::from("No token provided")))
    } else if valid_session(&mut database, e.clone()).await {
        let user = get_user_by_token(&mut database, e.clone()).await.unwrap();
        if cfg!(debug_assertions) {
            println!("./home/{}/{}", user.name, path.0);
        }
        if async_std::fs::metadata(format!("./home/{}/{}", user.name, path.0))
            .await
            .unwrap()
            .is_dir()
        {
            match async_std::fs::remove_dir_all(format!("./home/{}/{}", user.name, path.0)).await {
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
                    insert(&mut database, user.id, ActionType::Delete).await;
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
                    insert(&mut database, user.id, ActionType::Delete).await;
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
        Ok(HttpResponse::BadRequest().body("The token provided isn't valid"))
    }
}
