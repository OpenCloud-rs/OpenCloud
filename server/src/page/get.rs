use std::collections::BTreeMap;
use actix_web::{HttpRequest, HttpResponse as Response};
use crate::lib::file::file::{get_dir};
use crate::lib::archive::archive::*;
use actix_http::body::Body;

pub async fn cli(req: HttpRequest) -> std::io::Result<Response<Body>> {
    println!("{:?} ---",req.query_string());
    let result;

    let mut bvec: BTreeMap<&str, &str> = BTreeMap::new();
    let vec: Vec<&str> = req.query_string().split(|c| c == '&').collect();

    for i in 0..vec.len() {
        if let Some(_u) = vec[i].rfind("=") {
            let e: Vec<&str> = vec[i].split("=").collect();
            if e[0].is_empty() {
                continue;
            }
            bvec.insert(e[0], e[1]);
            continue;
        }
        if vec[i].is_empty() {
            continue;
        }
        bvec.insert(vec[i], &"");
    }
    if bvec.contains_key("download") {
        match bvec.get("download").unwrap().as_ref() {
            "tar.gz" => {
                result = get_tar(&req).await;
            }
            _ => {
                result = get_zip(&req).await;
            }
        }
    } else {
        result = get_dir(&req);
    }
    result
}

