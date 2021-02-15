/*use sqlx::Executor;

pub async fn _delete(id: i32) -> bool {
    let mut conn = conn().await;
    match conn
        .execute(format!("delete from log where id = {}", id).as_str())
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
*/
