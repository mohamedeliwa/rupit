use std::{env, ffi::OsString};

pub fn run_command(command: &str) -> () {
    println!("\nexecuting command: {}...\n", command);
    if cfg!(windows) {
        run_command_windows(command);
    } else {
        run_command_unix(command);
    }
    println!("\nRupit finished executing command: {}\n", command);
}

fn run_command_windows(command: &str) -> () {
    std::process::Command::new("cmd")
        .args(["/C", command])
        .status()
        .expect(&format!("{} command failed to execute", command));
}

fn run_command_unix(command: &str) -> () {
    // getting the name of the default shell from os env variables
    // adding support to different shells in unix and macos systems
    let shell = env::var_os("SHELL").unwrap_or_else(|| OsString::from(String::from("sh")));

    std::process::Command::new(shell)
        .arg("-c")
        .arg(command)
        .status()
        .expect(&format!("{} command failed to execute", command));
}
