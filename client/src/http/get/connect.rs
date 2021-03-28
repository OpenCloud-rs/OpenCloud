use crate::library::lib::Account;
use crate::Msg;
use seed::log;
use seed::prelude::{Header, Method, Request};
use seed::window;

pub async fn get_token(account: Account) -> Msg {
    let ip = format!(
        "{}{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string()),
        "/api/user/login"
    );
    let request = Request::new(ip.as_str())
        .method(Method::Post)
        .header(Header::custom("Access-Control-Allow-Origin", "*"))
        .header(Header::custom("Content-Type", "application/json"))
        .json(&account)
        .unwrap()
        .fetch()
        .await;

    match request {
        Ok(r) => {
            if r.status().code == 200 {
                match r.text().await {
                    Ok(s) => Msg::Token(Ok(s)),
                    Err(_) => Msg::Token(Err("Error".to_string())),
                }
            } else {
                Msg::Token(Err("Error token".to_string()))
            }
        }
        Err(_) => Msg::Token(Err("Error: Request Failed".to_string())),
    }
}
