use super::hash_password;
use datagn::DatabasePool;

pub async fn insert_user(
    database: &mut DatabasePool,
    name: String,
    email: String,
    password: String,
) -> std::io::Result<usize> {
    match database
        .execute_with_bind(
            "INSERT INTO User (name,email, password) VALUES(?1, ?2, ?3)",
            &[name, email, hash_password(password)],
        )
        .await
    {
        Ok(_) => Ok(usize::from(true)),
        Err(_) => Ok(usize::from(false)),
    }
}
