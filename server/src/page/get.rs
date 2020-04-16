use actix_web::{Error, HttpRequest, HttpResponse,};
use actix_web::dev::BodyEncoding;
use actix_web::http::ContentEncoding;
use shared::Folder;

pub async fn cli(req: HttpRequest) -> Result<HttpResponse, Error> {
    crate::lib::http::log(&req);
    let url = crate::lib::http::without_cli(req.path());

    let path = crate::lib::file::dir_content(url);
    let mut folder = Folder {
        result: true,
        lenght: path.len() as i64,
        content: path.to_owned(),
    };
    let start: String = String::from("Error");
    if path.starts_with(&[start]) {
        folder.result = false;
    }

    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(serde_json::to_string(&folder).unwrap()))
}