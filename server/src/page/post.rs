use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::create_home::create_home;
use crate::lib::db::user::get::{get_id_of_user, get_user_by_token};
use crate::lib::db::user::insert::insert_user;
use crate::lib::db::user::model::{LoginUser, MinimalUser};
use crate::lib::db::user::token::generate_token;
use crate::lib::db::user::update::update_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::{db::log::insert::insert, http::get_args};
use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use async_std::io::prelude::WriteExt;
use datagn::DatabasePool;

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

#[post("/user/create")]
pub async fn create_user(
    body: web::Json<MinimalUser>,
    data: web::Data<DatabasePool>,
) -> Result<HttpResponse, Error> {
    let mut database = data.get_ref().clone();
    match insert_user(
        &mut database,
        body.name.clone(),
        body.clone().email.unwrap_or_default(),
        body.password.clone(),
    )
    .await
    {
        Ok(_) => {
            let e = create_home(body.name.clone()).await;
            Ok(HttpResponse::Ok().body(e.body))
        }
        Err(_) => Ok(HttpResponse::BadRequest().body("Your request is bad")),
    }
}

#[post("/user/login")]
pub async fn login_user(body: web::Json<LoginUser>, data: web::Data<DatabasePool>) -> HttpResponse {
    let mut database = data.get_ref().clone();
    let token = generate_token();
    if cfg!(debug_assertions) {
        println!("name : {}, password: {}", body.name, body.password);
    }
    if let Some(id) = get_id_of_user(&mut database, body.name.clone(), body.password.clone()).await
    {
        update_token(&mut database, token.clone(), id.to_owned()).await;
        if cfg!(debug_assertions) {
            println!("{}", valid_session(&mut database, token.clone()).await);
        }
        HttpResponse::Ok().body(&token)
    } else {
        HttpResponse::BadRequest().body("No user was found")
    }
}
