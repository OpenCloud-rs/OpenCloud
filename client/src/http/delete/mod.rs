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

    let request = reqwest::Client::new().delete(ip)
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "application/json")
        .header("Token", token.as_str());

    match request.send().await {
        Ok(_) => Msg::DeleteFile(Ok(200), "Delete successfully".to_string()),
        Err(_) => Msg::DeleteFile(Err(-0), "Delete unsuccessfully".to_string()),
    }
}
