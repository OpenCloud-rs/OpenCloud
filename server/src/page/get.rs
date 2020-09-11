use actix_web::{HttpRequest, HttpResponse as Response,get, web};
use crate::lib::file::file::{get_dir};
use crate::lib::{http::http::get_args, archive::archive::*};
use actix_http::body::Body;
#[get("/api/file/{tokio:.*}/{path:.*}")]
pub async fn cli(req: HttpRequest, path: web::Path<(String, String)>) -> std::io::Result<Response<Body>> {
    println!("/{}",path.1);
    let result;

    let bvec = get_args(req.clone());

    if bvec.contains_key("download") {
        match bvec.get("download").unwrap().as_ref() {
            "tar.gz" => {
                result = get_tar(req.clone()).await;
            }
            _ => {
                result = get_zip(req.clone()).await;
            }
        }
    } else {
        result = get_dir(format!("/{}",path.1));
    }
    result
}

