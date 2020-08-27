use actix_web::HttpRequest;
use std::collections::BTreeMap;

pub fn without_api(string: &str) -> &str {
    string
        .char_indices()
        .next()
        .and_then(|(i, _)| string.get(i + 4..))
        .unwrap_or(&"")
}
/*pub fn log(request: &HttpRequest) {
    println!(
        "Nouvel utilisateur sur {} , Ip : {}",
        request.path(),
        request.connection_info().remote().unwrap()
    )
}*/

pub fn last_cli(req: HttpRequest) -> String {
    let split: Vec<&str> = req.path().split("/").collect();
    let result: String;
    if split.last().unwrap().is_empty() {
        result = String::from(split[split.len() - 2].to_owned())
    } else {
        result = String::from(split.last().take().unwrap().to_owned())
    }
    result
}

pub fn get_args(req: HttpRequest) -> BTreeMap<String, String> {
    let mut bvec: BTreeMap<String, String> = BTreeMap::new();
    let vec: Vec<&str> = req.query_string().split(|c| c == '&').collect();
    for i in 0..vec.len() {
        if let Some(_u) = vec[i].rfind("=") {
            let e: Vec<&str> = vec[i].split("=").collect();
            if e[0].is_empty() {
                continue;
            }
            bvec.insert(e[0].to_string(), e[1].to_string());
            continue;
        }
        if vec[i].is_empty() {
            continue;
        }
        bvec.insert(vec[i].to_string(), String::from(""));
    }
    bvec
}