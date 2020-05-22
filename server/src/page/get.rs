use crate::lib::http::{last_cli, without_cli};
use actix_files::file_extension_to_mime;
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use actix_web::{web, Error, HttpRequest, HttpResponse as Response};
use futures::{StreamExt, AsyncReadExt};
use actix_web::body::BodyStream;
use futures::future::IntoStream;
use crate::lib::file::get_file_as_byte_vec;
use actix_utils::mpsc;
use bytes::Bytes;
use std::collections::HashMap;

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
        println!("{:?}", item);
        match String::from_utf8(item.to_vec()) {
            Ok(o) => {
                let vec: Vec<&str> = o.split(|c| c == '=' || c == '&').collect();
                println!("{:?}", vec);
                if vec[0] == "type" {
                    match vec[1] {
                        "download" => {
                            if vec[3] == "zip" {
                                let (tx, rx_body) = mpsc::channel();
                                let _ = tx.send(Ok::<_, Error>(Bytes::from(get_file_as_byte_vec(without_cli(String::from(req.path()).as_ref()).parse().unwrap()))));
                             result = Ok(Response::Ok()
                                    .header("Access-Control-Allow-Origin", "*")
                                    .header("charset", "utf-8")
                                    .header("Content-Disposition", "attachment")
                                    .header("filename", format!("{}{}",last_cli(req.clone()), "dd.zip"))
                                    .content_type(
                                        file_extension_to_mime(without_cli(req.clone().path()))
                                            .essence_str(),
                                    )
                                    .encoding(ContentEncoding::Gzip).streaming(rx_body)
                                );
                            } else {
                                result = Ok(Response::Ok()
                                    .header("Access-Control-Allow-Origin", "*")
                                    .header("charset", "utf-8")
                                    .header("Content-Disposition", "attachment")
                                    .header("filename", format!("{}{}",last_cli(req.clone()), ".tar.gz"))
                                    .content_type(file_extension_to_mime(without_cli(req.clone().path())).essence_str())
                                    .encoding(ContentEncoding::Gzip).body("dd"))
                            }
                        }

                        _ => {
                            result = Ok(Response::Ok()
                                .header("Access-Control-Allow-Origin", "*")
                                .header("charset", "utf-8")
                                .content_type("application/json")
                                .encoding(ContentEncoding::Gzip)
                                .body("d"));
                        }
                    };
                }
            }
            Err(_t) => {
                println!("Error");
            }
        }
    }
    result
}
