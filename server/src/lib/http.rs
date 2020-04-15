
    use actix_web::HttpRequest;

    pub fn without_cli(string: &str) -> &str {
        string
            .char_indices()
            .next()
            .and_then(|(i, _)| string.get(i + 4..))
            .unwrap_or("")
    }
    pub fn log(request: &HttpRequest) {
        println!(
            "Nouvel utilisateur sur {} , Ip : {}",
            request.path(),
            request.connection_info().remote().unwrap()
        )
    }

