use std::process::exit;
pub fn break_point(exit_code: u8,exit_message: &str) -> &str {
    println!("{}",exit_message);
    exit(exit_code as i32);
}