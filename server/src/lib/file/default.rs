use actix_web::web::HttpResponse;

#[cfg(feature = "webclient")]
use include_flate::flate;

#[cfg(feature = "webclient")]
flate!(pub static INDEX: str from "../client/index.html");
#[cfg(feature = "webclient")]
flate!(pub static PACKAGE_JS: str from "../client/pkg/package.js");
#[cfg(feature = "webclient")]
flate!(pub static PACKAGE_BG: [u8]  from "../client/pkg/package_bg.wasm");
#[cfg(feature = "webclient")]
flate!(pub static BULMA_JS: str from "../client/pkg/bulma/bulma.js");
#[cfg(feature = "webclient")]
flate!(pub static BULMA_MIN_CSS: str from "../client/pkg/bulma/bulma.min.css");
#[cfg(feature = "webclient")]
flate!(pub static FILE_SVG: str from "../client/pkg/obj/file.svg");
#[cfg(feature = "webclient")]
flate!(pub static FOLDER_SVG: str from "../client/pkg/obj/folder.svg");

#[cfg(feature = "webclient")]
pub async fn indexhtml() -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(format!("{}", *INDEX))
}

#[cfg(not(feature = "webclient"))]
pub async fn indexhtml() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn wasmloader() -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "application/javascript")
        .body(format!("{}", *PACKAGE_JS))
}

#[cfg(not(feature = "webclient"))]
pub async fn wasmloader() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn wasm() -> HttpResponse {
    let body = Vec::from(PACKAGE_BG.clone());
    HttpResponse::Ok()
        .header("Content-Type", "application/wasm")
        .body(body)
}

#[cfg(not(feature = "webclient"))]
pub async fn wasm() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn bulma() -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "text/css")
        .body(format!("{}", *BULMA_MIN_CSS))
}

#[cfg(not(feature = "webclient"))]
pub async fn bulma() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn bulma_js() -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "application/javascript")
        .body(format!("{}", *BULMA_JS))
}

#[cfg(not(feature = "webclient"))]
pub async fn bulma_js() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn file_svg() -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "image/svg+xml")
        .body(format!("{}", *FILE_SVG))
}

#[cfg(not(feature = "webclient"))]
pub async fn file_svg() -> HttpResponse {
    disable()
}

#[cfg(feature = "webclient")]
pub async fn folder_svg() -> HttpResponse {
    if cfg!(feature = "webclient") {
    HttpResponse::Ok()
        .header("Content-Type", "image/svg+xml")
        .body(format!("{}", *FOLDER_SVG))} else {
            disable()
        }
}

#[cfg(not(feature = "webclient"))]
pub async fn folder_svg() -> HttpResponse {
    disable()
}

pub fn disable() -> HttpResponse {
    HttpResponse::BadRequest().body("The webclient feature is disable")
}
