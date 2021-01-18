use crate::lib::db::log::insert::insert;
use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::get::{get_id_of_user, get_user_by_token};
use crate::lib::db::user::model::LoginUser;
use crate::lib::db::user::token::gen_token;
use crate::lib::db::user::update::update_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::file::file::{get_dir, get_file_preview, Sort};
use crate::lib::{archive::archive::*, http::http::get_args};
use actix_http::body::Body;
use actix_web::{get, post, web, HttpRequest, HttpResponse as Response, HttpResponse};

#[get("/file/{path:.*}")]
pub async fn cli(req: HttpRequest, path: web::Path<String>) -> std::io::Result<Response<Body>> {
    let result;
    let e = if let Some(e) = req.headers().get("token") {
        String::from(e.to_str().unwrap_or_default())
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };
    if !e.is_empty() {
        if valid_session(e.clone()).await {
            let bvec = get_args(req.clone());
            let user = match get_user_by_token(e.clone()).await {
                Some(e) => e,
                None => {
                    return Ok(HttpResponse::Ok().body(String::from("Error on get user")));
                }
            };
            if bvec.contains_key("download") {
                match bvec.get("download").unwrap_or(&String::new()).as_ref() {
                    "tar.gz" => {
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
            insert(user.id, ActionType::Get).await;
        } else {
            result = Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }
    } else {
        result = Ok(HttpResponse::Ok().body(String::from("No token provided")));
    }

    result
}

#[post("/user/login")]
pub async fn login_user(body: web::Json<LoginUser>) -> std::io::Result<Response<Body>> {
    let token = gen_token();
    println!("name : {}, password: {}", body.name, body.password);
    if let Some(id) = get_id_of_user(body.name.clone(), body.password.clone()).await {
        update_token(token.clone(), id.to_owned()).await;
        println!("{}", valid_session(token.clone()).await);
        Ok(HttpResponse::Ok().body(&token))
    } else {
        Ok(HttpResponse::Ok().body("No user was found"))
    }
}

pub async fn default_api_handler() -> std::io::Result<HttpResponse> {
    Ok(HttpResponse::BadRequest().body("Bad Usage of Api"))
}
