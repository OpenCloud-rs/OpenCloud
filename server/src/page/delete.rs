use actix_web::{Error, HttpRequest, HttpResponse};
use shared::Folder;

pub async fn deletef(req: HttpRequest) -> Result<HttpResponse, Error> {
    let to_delete = crate::lib::http::without_cli(req.path());
    let mut result = Folder{
        result: false,
        lenght: 0,
        content: vec![String::new()],

    };
    match std::fs::remove_dir(to_delete) {
        Ok(_o) => {
            result.result = true;
            result.content = vec!["Works".to_string()]
        },
        Err(_e) => {
            result.content = vec!["Error".to_string()]
        },
    };
    Ok(HttpResponse::Ok()
        .header("charset", "utf-8")
        .header("Access-Control-Allow-Origin", "*")
        .body(serde_json::to_string(&result).unwrap()))
}