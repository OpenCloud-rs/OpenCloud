use std::io::Write;
use tokio::stream::StreamExt;

use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{guard, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use shared::Folder;
const CLIENT_PAGE: &str = "./client/index.html";

async fn cli(req: HttpRequest) -> Result<HttpResponse, Error> {
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

    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("charset", "utf-8")
        .content_type("application/json")
        .encoding(ContentEncoding::Gzip)
        .body(serde_json::to_string(&folder).unwrap())
        .into())
}
async fn client() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open(CLIENT_PAGE)?.set_status_code(StatusCode::NOT_FOUND))
}
async fn save_file(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    let url = crate::lib::http::without_cli(req.path());
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("{}/{}", url, filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn delete(req: HttpRequest) -> Result<HttpResponse, Error> {
    let to_delete = crate::lib::http::without_cli(req.path());
    let matched = match std::fs::remove_dir(to_delete) {
        Ok(_o) => {
            String::from("Ok it's delete")
        }
        Err(_e) => {
            String::from("Error")
        }
    };
    Ok(HttpResponse::Ok().body(matched))
}
