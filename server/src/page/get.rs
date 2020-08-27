use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse as Response};
use crate::lib::file::file::{get_dir};
use crate::lib::{http::http::get_args, archive::archive::*};
use actix_http::body::Body;

pub async fn cli(req: HttpRequest) -> std::io::Result<Response<Body>> {
    println!("{:?} ---",req.query_string());
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
        result = get_dir(req.clone());
    }
    result
}

