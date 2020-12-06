use seed::prelude::{Header, Method, Request};
use seed::window;

use crate::Msg;

pub async fn delete(token: String, name: String) -> Msg {
    let ip = format!(
        "{}{}{}{}",
        "http://".to_owned(),
        &window().location().host().expect("127.0.0.1:8081"),
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
    let result = match e {
        Ok(_) => Ok(200),
        Err(_) => Err(500),
    };

    Msg::DeleteFile(result, "name".to_string())
}
