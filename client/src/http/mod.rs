use seed::window;

pub mod delete;
pub mod get;
pub mod log;
pub mod post;
pub mod upload;

pub fn get_ip() -> String {
    format!(
        "{}{}",
        "http://".to_owned(),
        &window()
            .location()
            .host()
            .unwrap_or("127.0.0.1:8081".to_string()),
    )
}
