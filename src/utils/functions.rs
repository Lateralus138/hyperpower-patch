use std::{env::{var_os},process::exit,ffi::OsString,};
pub fn var_os_or_exit(var_name: &str,exit_code: u8) -> OsString {
    let env_home = match var_os(var_name) {
        Some(value)=> value,
        None => {
            println!("${} environment variable not found.",var_name);
            exit(exit_code as i32);
        }
    };
    env_home
}