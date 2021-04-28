use crate::library::lib::Account;
use crate::Msg;
use seed::prelude::{FetchError, Header, Method, Request};
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

    let request = reqwest::Client::new()
                            .post(ip.as_str()).json(&account)
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Content-Type", "application/json");

    match request.send().await {
        Ok(r) => {
            if r.status().code == 200 {
                match r.text().await {
                    Ok(s) => Msg::Token(Ok(s)),
                    Err(e) => {
                        Msg::Token(Err((Some(r.status().code as i32), e.to_string())))
                    }
                }
            } else {
                Msg::Token(Err((
                    Some(r.status().code as i32),
                    e.to_string()),
                ))
            }
        }
        Err(e) => Msg::Token(Err((None, e.to_string()))),
    }
}