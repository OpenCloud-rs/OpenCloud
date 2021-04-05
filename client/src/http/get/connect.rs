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
                    Err(e) => Msg::Token(Err((Some(r.status().code as i32), fetcherror_to_string(e)))),
                }
            } else {
                Msg::Token(Err((Some(r.status().code as i32), r.text().await.unwrap_or_else(|e| fetcherror_to_string(e)))))
            }
        }
        Err(e) => Msg::Token(Err((None, fetcherror_to_string(e)))),
    }
}

fn fetcherror_to_string(e: FetchError) -> String {
    match e {
        seed::prelude::FetchError::SerdeError(e) => {e.to_string()}
        seed::prelude::FetchError::DomException(e) => {e.message()}
        seed::prelude::FetchError::PromiseError(e) => {e.as_string().unwrap_or("Error on parse message".to_string())}
        seed::prelude::FetchError::NetworkError(e) => {e.as_string().unwrap_or("Error on parse message".to_string())}
        seed::prelude::FetchError::RequestError(e) => {e.as_string().unwrap_or("Error on parse message".to_string())}
        seed::prelude::FetchError::StatusError(e) => {e.code.to_string()}
    }
}