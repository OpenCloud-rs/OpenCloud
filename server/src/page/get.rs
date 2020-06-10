use std::collections::BTreeMap;

use actix_files::file_extension_to_mime;
use actix_utils::mpsc;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{Error, HttpRequest, HttpResponse as Response};
use bytes::Bytes;

use crate::lib::file::get_file_as_byte_vec;
use crate::lib::http::last_cli;

pub async fn cli(req: HttpRequest) -> Result<Response, Error> {
    crate::lib::http::log(&req);
    let mut result = Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(crate::lib::file::dir_content(&req)));

    let mut bvec: BTreeMap<&str, &str> = BTreeMap::new();
    let vec: Vec<&str> = req.query_string().split(|c| c == '&').collect();

    for i in 0..vec.len() {
        if let Some(u) = vec[i].rfind("=") {
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

    if bvec.contains_key("type") {
        if bvec.get("type").unwrap() == &"download" {
            if bvec.contains_key("download_type") {
                match bvec.get("download_type").unwrap().as_ref() {
                    "zip" => {
                        let (tx, rx_body) = mpsc::channel();
                        let _ = tx.send(Ok::<_, Error>(Bytes::from(get_file_as_byte_vec(
                            req.path().parse().unwrap(),
                        ))));
                        result = Ok(Response::Ok()
                            .header("Access-Control-Allow-Origin", "*")
                            .header("charset", "utf-8")
                            .header(
                                "Content-Disposition",
                                format!(
                                    "\"attachment\";filename=\"{}.zip\"",
                                    last_cli(req.clone())
                                ),
                            )
                            .content_type(file_extension_to_mime(req.clone().path()).essence_str())
                            .encoding(ContentEncoding::Gzip)
                            .streaming(rx_body));
                    }
                    _ => {
                        let (tx, rx_body) = mpsc::channel();
                        let _ = tx.send(Ok::<_, Error>(Bytes::from(get_file_as_byte_vec(
                            req.path().parse().unwrap(),
                        ))));
                        result = Ok(Response::Ok()
                            .header("Access-Control-Allow-Origin", "*")
                            .header("charset", "utf-8")
                            .header(
                                "Content-Disposition",
                                format!("attachment;filename=\"{}.zip\"", last_cli(req.clone())),
                            )
                            .content_type(file_extension_to_mime(req.clone().path()).essence_str())
                            .encoding(ContentEncoding::Gzip)
                            .streaming(rx_body));
                    }
                }
            }
        }
    }
    result
}
