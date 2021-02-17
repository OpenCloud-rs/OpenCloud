use crate::lib::db::log::insert::insert;
use crate::lib::db::log::model::ActionType;
use crate::lib::db::user::get::get_user_by_token;
use crate::lib::db::user::valid_session::valid_session;
use crate::lib::file::{get_dir, get_file_preview, Sort};
use crate::lib::{archive::*, http::get_args};
use actix_web::{get, web, HttpRequest, HttpResponse};
use datagn::DatabasePool;

#[get("/file/{path:.*}")]
pub async fn cli(
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
        insert(&mut database, user.id, ActionType::Get).await;
    } else {
        result = HttpResponse::BadRequest().body("The token provided isn't valid")
    }

    result
}

pub async fn default_api_handler() -> HttpResponse {
    HttpResponse::BadRequest().body("Bad Usage of Api")
}

pub async fn default_404() -> HttpResponse {
    HttpResponse::NotFound().body("Oh no, file not found")
}
