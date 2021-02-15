use datagn::DatabasePool;

pub async fn update_token(database: &mut DatabasePool, token: String, id: i32) -> bool {
    match database
        .execute_with_bind(
            "UPDATE User SET token=?1 WHERE id=?2",
            &[token, id.to_string()],
        )
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
