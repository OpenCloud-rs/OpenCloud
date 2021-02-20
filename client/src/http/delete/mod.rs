use seed::prelude::{Header, Method, Request};
use seed::window;

use crate::Msg;

pub async fn delete(token: String, name: String) -> Msg {
    let ip = format!(
        "{}{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string()),
        "/api/file/",
        name
    );
    let e = Request::new(ip.as_str())
        .method(Method::Delete)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .header(Header::custom("Token", token.as_str()))
        .fetch()
        .await;
    match e {
        Ok(_) => Msg::DeleteFile(Ok(200), "Delete successfully".to_string()),
        Err(_) => Msg::DeleteFile(Err(500), "Delete unsuccessfully".to_string()),
    }
}
