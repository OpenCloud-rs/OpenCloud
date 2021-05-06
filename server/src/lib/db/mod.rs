use datagn::DatabasePool;

pub mod log;
pub mod user;

pub async fn create_db(database: &mut DatabasePool) {
    log::create::create(database).await;
    user::create::create(database).await;
}
