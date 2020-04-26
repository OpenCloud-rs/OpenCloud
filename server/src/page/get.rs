use crate::lib::file::compress;
use actix_web::dev::{BodyEncoding, PayloadStream};
use actix_web::http::ContentEncoding;
use actix_web::{web, Error, HttpRequest, HttpResponse as Response};
use futures::StreamExt;
use actix_http::body::BodyStream;

pub async fn cli(req: HttpRequest, mut body: web::Payload) -> Result<Response, Error> {
    crate::lib::http::log(&req);
    let mut result = Ok(Response::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(crate::lib::file::dir_content(&req)));
    while let Some(item) = body.next().await {
        let item = item?;
        match String::from_utf8(item.to_vec()) {
            Ok(o) => {
                let vec: Vec<&str> = o.split('=').collect();
                if vec[0] == "type" {
                    match vec[1] {
                        "download" => {
                            /*result = Ok(Response::Ok()
                                .header("Access-Control-Allow-Origin", "*")
                                .header("charset", "utf-8")
                                .content_type(
                                    "Content-Disposition: attachment; filename=\"MyFileName.ext\"",
                                )
                                .encoding(ContentEncoding::Gzip).streaming());*/
                            result = Ok(Response::Ok()
                                .header("Access-Control-Allow-Origin", "*")
                                .header("charset", "utf-8")
                                .content_type("application/json")
                                .encoding(ContentEncoding::Gzip)
                                .body(crate::lib::file::dir_contentb(&req)));
                        }

                        _ => {
                            result = Ok(Response::Ok()
                                .header("Access-Control-Allow-Origin", "*")
                                .header("charset", "utf-8")
                                .content_type("application/json")
                                .encoding(ContentEncoding::Gzip)
                                .body(crate::lib::file::dir_contentb(&req)));
                        }
                    };
                }
            }
            Err(_t) => {
                println!("Error");
            }
        };
    }
    result
}
