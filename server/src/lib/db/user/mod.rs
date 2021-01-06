pub mod create;
pub mod create_home;
pub mod get;
pub mod insert;
pub mod model;
pub mod token;
pub mod update;
pub mod valid_session;

use whirlpool::{Whirlpool, Digest};


pub fn hash_password(password: String) -> String {
    format!("{:x}", Whirlpool::new().chain(password.as_bytes()).finalize())
}