use actix_web::{get, Responder};
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
    let start: String = "Error".parse().unwrap();
    if path.starts_with(&[start]) {
        folder.result = false;
    }

    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .content_type("application/json")
        .body(serde_json::to_string(&folder).unwrap())
}

