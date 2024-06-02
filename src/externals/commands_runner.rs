use crate::{services::Runner, Result};

use std::{env, ffi::OsString};

pub struct CommandRuner;

impl CommandRuner {
    fn run_command_windows(&self, command: &str) -> Result<()> {
        std::process::Command::new("cmd")
            .args(["/C", command])
            .status()?;
        Ok(())
    }

    fn run_command_unix(&self, command: &str) -> Result<()> {
        // getting the name of the default shell from os env variables
        // adding support to different shells in unix and macos systems
        let shell = env::var_os("SHELL").unwrap_or_else(|| OsString::from(String::from("sh")));

        std::process::Command::new(shell)
            .arg("-c")
            .arg(command)
            .status()?;
        Ok(())
    }
}

impl Runner for CommandRuner {
    fn run(&self, command: &str) -> Result<()> {
        if cfg!(windows) {
            self.run_command_windows(command)?;
        } else {
            self.run_command_unix(command)?;
        }
        Ok(())
    }
}
