use std::{env, ffi::OsString, io};

pub fn run_command(command: &str) -> Result<(), io::Error> {
    if cfg!(windows) {
        run_command_windows(command)?;
    } else {
        run_command_unix(command)?;
    }
    Ok(())
}

fn run_command_windows(command: &str) -> Result<(), io::Error> {
    std::process::Command::new("cmd")
        .args(["/C", command])
        .status()?;
    Ok(())
}

fn run_command_unix(command: &str) -> Result<(), io::Error> {
    // getting the name of the default shell from os env variables
    // adding support to different shells in unix and macos systems
    let shell = env::var_os("SHELL").unwrap_or_else(|| OsString::from(String::from("sh")));

    std::process::Command::new(shell)
        .arg("-c")
        .arg(command)
        .status()?;
    Ok(())
}
