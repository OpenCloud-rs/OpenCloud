use actix_web::HttpRequest;
use std::collections::BTreeMap;

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
