use crate::library::lib::Account;
use crate::Msg;
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
        .post(ip.as_str())
        .json(&account)
        .header("Access-Control-Allow-Origin", "*");

    match request.send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            let text = r.text();
            if status == 200 {
                match text.await {
                    Ok(s) => Msg::Token(Ok(s)),
                    Err(e) => Msg::Token(Err((Some(status as i32), e.to_string()))),
                }
            } else {
                Msg::Token(Err((
                    Some(status as i32),
                    text.await.unwrap_or_else(|_| String::new()),
                )))
            }
        }
        Err(e) => Msg::Token(Err((None, e.to_string()))),
    }
}
