use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{get, Responder};
use actix_web::{guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use shared::Folder;
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
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(serde_json::to_string(&folder).unwrap())
}
async fn client() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("./client/index.html")?.set_status_code(StatusCode::NOT_FOUND))
}
