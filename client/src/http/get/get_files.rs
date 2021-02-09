use crate::{http::get_ip, Msg};
use seed::log;
use shared::JsonStruct;
pub async fn get_files(from: String, token: String) -> Msg {
    let reqwest = reqwest::Client::new()
        .get(format!("{}/api/file/{}", get_ip(), from).as_str())
        .header("Token", token.clone())
        .send()
        .await;

    let json = match reqwest {
        Ok(e) => match e.json::<JsonStruct>().await {
            Ok(json) => json,
            Err(e) => {
                log!(format! {"{:?}", e});
                JsonStruct::default()
            }
        },
        Err(e) => {
            log!(format! {"{:?}", e});
            JsonStruct::default()
        }
    };

    Msg::Fetched(Some(json))
}

pub fn back(url: String) -> String {
    let ur: Vec<&str> = url.split("/").collect();
    let mut n = 1;
    let mut result = String::new();
    log!(ur.len() - 1);
    for u in ur.clone() {
        if n == ur.len() - 1 {
            break;
        } else {
            result.push_str(format!("{}/", u).as_str());
        }
        n += 1;
    }
    result
}
