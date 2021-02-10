use crate::lib::db::conn::conn;
use crate::lib::db::user::hash_password;
use sqlx::Executor;

pub async fn insert_user(name: String, email: String, password: String) -> std::io::Result<usize> {
    let mut conn = conn().await;
    match conn
        .execute(
            format!(
                "INSERT INTO User (name,email, password) VALUES(\"{}\", \"{}\", \"{}\")",
                name,
                email,
                hash_password(password)
            )
            .as_ref(),
        )
        .await
    {
        Ok(_) => Ok(usize::from(true)),
        Err(_) => Ok(usize::from(false)),
    }
}
