use actix_web::{Error, HttpRequest, HttpResponse as Response, web};
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use futures::StreamExt;

pub async fn cli(req: HttpRequest, mut body: web::Payload) -> Result<Response, Error> {
    crate::lib::http::log(&req);
    let mut bod = crate::lib::file::dir_content(&req);
    while let Some(item) = body.next().await {
        let item = item?;
        match String::from_utf8(item.to_vec()) {
            Ok(o) => {
                let vec: Vec<&str> = o.split('=').collect();
                if vec[0] == "type" {
                    match vec[1] {
                        "download" => bod = String::from("Download"),
                        _ => bod = crate::lib::file::dir_content(&req),
                    };
                }
            },
            Err(_t) => {println!("Error")}
        }
    }
    Ok(Response::Ok().header("Access-Control-Allow-Origin", "*").header("charset", "utf-8").content_type("application/json").encoding(ContentEncoding::Gzip).body(bod))
}