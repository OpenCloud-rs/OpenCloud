use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::user::insert::insert_user;
use crate::lib::db::user::model::MinimalUser;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::{db::log::insert::insert, http::get_args};
use crate::lib::{db::log::model::ActionType, log::error};
use crate::lib::{db::user::create_home::create_home, log::info};
use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use async_std::io::prelude::WriteExt;
use tokio_stream::StreamExt;

#[post("/file/{path:.*}")]
pub async fn save_file(
    req: HttpRequest,
    mut payload: Multipart,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or(""))
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    if e.is_empty() {
        Ok(HttpResponse::Ok().body("No token provided"))
    } else {
        let url = format!("/{}", path.0);
        if valid_session(e.clone()).await {
            let user = match get_user_by_token(e.clone()).await {
                Some(e) => e,
                None => {
                    return Ok(HttpResponse::Ok().body("Can't get user"));
                }
            };
            insert(user.id, ActionType::Upload).await;
            while let Ok(Some(mut field)) = payload.try_next().await {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(ToString::to_string))
                    .expect("Can't get field name!");
                info("No panic");
                let filepath = format!(
                    "./home/{}/{}/{}",
                    user.name,
                    url.strip_prefix("/").unwrap(),
                    filename
                );
                // File::create is blocking operation, use threadpool
                if cfg!(debug_assertions) {
                    println!(
                    "--------------------- Url : {}, Name: {}, Path: {} ---------------------------",
                    url, filename, filepath
                );
                }
                let mut f = async_std::fs::File::create(filepath.clone()).await.unwrap();
                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(e) => {
                            f = f.write_all(&e).await.map(|_| f).unwrap();
                        }
                        Err(e) => {
                            error(format!("{:?}", e));
                        }
                    }
                }
            }
            return Ok(HttpResponse::Ok().body("The file is uploaded"));
        } else {
            Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }
    }
}

#[post("/user/create")]
pub async fn create_user(body: web::Json<MinimalUser>) -> Result<HttpResponse, Error> {
    match insert_user(
        String::from(body.name.clone()),
        String::from(body.clone().email.unwrap_or_default()),
        String::from(body.password.clone()),
    )
    .await
    {
        Ok(_) => {
            let e = create_home(body.name.clone()).await;
            if e.result {
                Ok(HttpResponse::Ok().body(e.body))
            } else {
                Ok(HttpResponse::Ok().body(e.body))
            }
        }
        Err(_) => Ok(HttpResponse::Ok().body("Your request is bad")),
    }
}
