use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http};
use actix_web::Result;

pub fn p500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
       .headers_mut()
       .insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res))
}
