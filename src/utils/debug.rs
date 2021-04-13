use std::any::type_name;
use std::process::exit;
use std::env::var_os;
pub fn type_of<T>(_: T) ->
    &'static str {type_name::<T>()}
pub fn break_point(exit_code: u8,exit_message: &str) {
    println!("{}",exit_message);
    exit(exit_code as i32);
}