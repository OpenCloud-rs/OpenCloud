use actix_http::Error;
use actix_web::web::HttpResponse;
use include_flate::flate;

flate!(pub static INDEX: str from "../client/index.html");
flate!(pub static PACKAGE_JS: str from "../client/pkg/package.js");
flate!(pub static PACKAGE_BG: [u8]  from "../client/pkg/package_bg.wasm");
flate!(pub static BULMA_JS: str from "../client/pkg/bulma/bulma.js");
flate!(pub static BULMA_MIN_CSS: str from "../client/pkg/bulma/bulma.min.css");
flate!(pub static FILE_SVG: str from "../client/pkg/obj/file.svg");
flate!(pub static FOLDER_SVG: str from "../client/pkg/obj/folder.svg");

pub async fn indexhtml() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(format!("{}", *INDEX)))
}

pub async fn wasmloader() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/javascript")
        .body(format!("{}", *PACKAGE_JS)))
}

pub async fn wasm() -> Result<HttpResponse, Error> {
    let body = Vec::from(PACKAGE_BG.clone());
    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/wasm")
        .body(body))
}

pub async fn bulma() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/css")
        .body(format!("{}", *BULMA_MIN_CSS)))
}

pub async fn bulma_js() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/javascript")
        .body(format!("{}", *BULMA_JS)))
}

pub async fn file_svg() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "image/svg+xml")
        .body(format!("{}", *FILE_SVG)))
}

pub async fn folder_svg() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", "image/svg+xml")
        .body(format!("{}", *FOLDER_SVG)))
}
