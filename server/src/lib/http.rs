use actix_web::HttpRequest;

pub fn without_api(string: &str) -> &str {
    string
        .char_indices()
        .next()
        .and_then(|(i, _)| string.get(i + 4..))
        .unwrap_or(&"")
}
pub fn log(request: &HttpRequest) {
    println!(
        "Nouvel utilisateur sur {} , Ip : {}",
        request.path(),
        request.connection_info().remote().unwrap()
    )
}

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
