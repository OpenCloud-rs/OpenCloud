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
            Ok(json) => Some(json),
            Err(e) => {
                log!(format! {"{:?}", e});
                None
            }
        },
        Err(e) => {
            log!(format! {"{:?}", e});
            None
        }
    };

    Msg::Fetched(json)
}

pub fn back(url: String) -> String {
    let ur: Vec<&str> = url.split("/").collect();
    let mut n = 1;
    let mut result = String::new();
    let lenght = ur.len() - 1;
    for u in ur {
        if n >= lenght {
            break;
        } else {
            result.push_str(format!("{}/", u).as_str());
        }
        n += 1;
    }
    result
}
