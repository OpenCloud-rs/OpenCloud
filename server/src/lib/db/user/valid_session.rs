use datagn::DatabasePool;
use actix_web::web::HttpRequest;
use crate::lib::http::get_args;

pub async fn valid_session(data: &mut DatabasePool, token: String) -> bool {
    let mut result = false;
    if !token.is_empty()
        && data
            .execute_with_bind("SELECT * FROM `User` WHERE token =?1", &[token])
            .await
            .is_ok()
    {
        result = true
    }
    result
}

pub async fn from_headers_if_valid_token_get_token(data: &mut DatabasePool, req: HttpRequest) -> Option<String> {
    let e = if let Some(e) = &req.headers().get("token") {
        String::from(e.to_str().unwrap_or_default())
    } else if let Some(e) = get_args(req.clone()).get("token") {
        String::from(e)
    } else {
        String::new()
    };

    if valid_session(data, e.clone()).await {
        Some(e)
    } else {
        None
    }

}