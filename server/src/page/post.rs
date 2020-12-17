use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::create_home::create_home;
use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::user::insert::insert_user;
use crate::lib::db::user::model::MinimalUser;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::{db::log::insert::insert, http::http::get_args};
use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use std::io::Write;
use tokio::stream::StreamExt;

#[post("/api/file/{path:.*}")]
pub async fn save_file(
    req: HttpRequest,
    mut payload: Multipart,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    println!("-----------------  {}  ---------------------------", path);
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().expect("Error to_str"))
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    if !e.is_empty() {
        let url = format!("/{}", path.0);
        if valid_session(e.clone()).await {
            let user = match get_user_by_token(e.clone()).await {
                Some(e) => e,
                None => {
                    return Ok(HttpResponse::Ok().body("Can't get user"));
                }
            };
            while let Some(mut field) = tokio::stream::StreamExt::try_next(&mut payload)
                .await
                .expect("Error")
            {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_name().map(ToString::to_string))
                    .expect("Can't get field name!");
                let filepath = format!("./home/{}/{}/{}", user.name, url, filename);
                // File::create is blocking operation, use threadpool
                println!(
                    "--------------------- Url : {}, Path: {} ---------------------------",
                    url, filepath
                );
                // let mut f = web::block(|| std::fs::File::create(filepath)).await;
                let mut f = match web::block(|| std::fs::File::create(filepath)).await {
                    Ok(e) => e,
                    Err(_) => {
                        return Ok(HttpResponse::Ok().body("Error on file upload"));
                    }
                };
                // Field in turn is stream of *Bytes* object
                while let Some(Ok(chunk)) = field.next().await {
                    // filesystem operations are blocking, we have to use threadpool
                    f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
                }
            }

            insert(user.id, ActionType::Upload).await;
            return Ok(HttpResponse::Ok().body("The file is uploaded"));
        } else {
            Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }
    } else {
        Ok(HttpResponse::Ok().body("No token provided"))
    }
}

#[post("/api/user/create")]
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
