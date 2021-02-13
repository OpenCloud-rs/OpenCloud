use datagn::DatabasePool;

pub async fn valid_session(data: &mut DatabasePool, token: String) -> bool {
    let mut result = false;
    if !token.is_empty() {
        match data.execute_with_bind("SELECT * FROM `User` WHERE token =?1",&[token]).await {
            Ok(_) => result = true,
            Err(_) => {}
        };
    }
    result
}
