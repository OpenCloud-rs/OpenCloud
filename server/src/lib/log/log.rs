use std::fmt::Display;

use colorful::{Color, Colorful};
pub fn warn<A: Display>(msg: A) {
    let string = format!("[WARNING] {}", msg);
    println!("{}", string.color(Color::Yellow))
}
pub fn info<A: Display>(msg: A) {
    let string = format!("[INFO] {}", msg);
    println!("{}", string.color(Color::Blue))
}

pub fn error<A: Display>(msg: A) {
    let string = format!("[ERROR] {}", msg);
    println!("{}", string.color(Color::Red))
}
