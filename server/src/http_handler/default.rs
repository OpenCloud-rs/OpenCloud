use actix_web::HttpResponse;

pub async fn default_api_handler() -> HttpResponse {
    HttpResponse::BadRequest().body("Bad Usage of Api")
}

pub async fn default_404() -> HttpResponse {
    HttpResponse::NotFound().body("Oh no, file not found")
}

use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::Result;
use actix_web::{dev, http};

pub fn p500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
