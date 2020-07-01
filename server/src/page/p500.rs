use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, http};

pub fn p500<T>(
    mut res: dev::ServiceResponse<T>,
) -> Result<ErrorHandlerResponse<T>, actix_http::error::Error> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("An 500 Server Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
