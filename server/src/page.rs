use actix_web::{get, Responder};
use shared::Folder;
use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use std::path::PathBuf;
use std::io::Read;
#[get("/cli/{path:.*}")]
async fn cli(req: HttpRequest) -> impl Responder {
    crate::lib::http::log(&req);
    let url = crate::lib::http::without_cli(req.path());

    let mut path = crate::lib::file::dir_content(url);
    let mut folder = Folder {
        result: true,
        lenght: path.len() as i64,
        content: (&mut path).to_owned(),
    };
    let start: String = String::from("Error");
    if path.starts_with(&[start]) {
        folder.result = false;
    }

    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset","utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(serde_json::to_string(&folder).unwrap())
}
async fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("./client/index.html")?.set_status_code(StatusCode::NOT_FOUND))
}
/*#[get("/{path:.*}")]
async fn client() -> impl Responder {

    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset","utf-8")
        .encoding(ContentEncoding::Gzip)
        .body(body)
}*/

// async fn index(req: HttpRequest) -> impl Responder {
//     let file = std::fs::File::open("../../client/index.html");
//     let mut body = String::new();
//     let mut dd = String::new();
//     for mut b in file {
//         b.read_to_string(&mut body).;
//     }
//     HttpResponse::Ok()
//         .body(body)
// }