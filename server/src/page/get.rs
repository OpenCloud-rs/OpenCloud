use crate::lib::db::user::get::{get_id, get_user_by_token};
use crate::lib::db::user::model::LoginUser;
use crate::lib::db::user::token::gen_token;
use crate::lib::db::user::update::update_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::file::file::get_dir;
use crate::lib::{archive::archive::*, http::http::get_args};
use actix_http::body::Body;
use actix_web::{get, web, HttpRequest, HttpResponse as Response, HttpResponse};

#[get("/api/file/{path:.*}")]
pub async fn cli(req: HttpRequest, path: web::Path<String>) -> std::io::Result<Response<Body>> {
    let result;
    if let Some(e) = req.headers().get("token") {
        if valid_session(String::from(e.to_str().expect("Parse Str Error"))) {
            let bvec = get_args(req.clone());
            let user = get_user_by_token(String::from(e.to_str().expect("Parse Str Error"))).expect("Error");
            println!("{}/{}",user.home, path.0.clone());
            if bvec.contains_key("download") {
                match bvec.get("download").unwrap().as_ref() {
                    "tar.gz" => {
                        result = get_tar(format!("{}/{}",user.home, path.0.clone())).await;
                    }
                    _ => {
                        result = get_zip(req.clone()).await;
                    }
                }
            } else {
                result = get_dir(format!("{}/{}",user.home, path.0.clone()));
            }
        } else {
            result = Ok(HttpResponse::Ok().body("The token provided isn't valid"))
        }
    } else {
        result = Ok(HttpResponse::Ok().body(String::from("No token provided")));
    }

    result
}

#[get("/api/user/login")]
pub async fn login_user(body: web::Json<LoginUser>) -> std::io::Result<Response<Body>> {
    let token = gen_token();
    println!("name : {}, password: {}", body.name, body.password);
    let id = get_id(body.name.clone(), body.password.clone());
    update_token(token.clone(), id);
    println!("{}", valid_session(token.clone()));
    Ok(HttpResponse::Ok().body(&token))
}

