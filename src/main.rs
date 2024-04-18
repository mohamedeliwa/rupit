use clap::Parser;
use config::ConfigError;
use serde_json::Value;
use std::{env, ffi::OsString};

mod settings;
use settings::Settings;

#[derive(Parser, Debug)]
struct Arguments {
    /// the alias for the command/s to execute
    alias: String,
}

fn main() -> Result<(), ConfigError> {
    let config_file_path = settings::get_config_file_path();

    let settings: Settings = settings::get_user_defined_settings(config_file_path).unwrap();

    // extracting the command alias from the cli arguments
    let args: Arguments = Arguments::parse();

    // getting the actual command from the config file
    let alias = args.alias;
    let command = settings.aliases[&alias].clone();

    match command {
        Value::String(command) => {
            println!("\nalias: {:?}", alias);
            println!("\nrunning command: {}...\n", command);

            if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(["/C", &command])
                    .status()
                    .expect(&format!("{} command failed to execute", &command));
            } else {
                // getting the name of the default shell from os env variables
                // adding support to different shells in unix and macos systems
                let shell =
                    env::var_os("SHELL").unwrap_or_else(|| OsString::from(String::from("sh")));

                std::process::Command::new(shell)
                    .arg("-c")
                    .arg(&command)
                    .status()
                    .expect(&format!("{} command failed to execute", &command));
            }

            println!("\nRupit finished executing command: {}\n", command);
            Ok(())
        }
        Value::Null => {
            panic!("\n{:?} alias not found in aliases list\n", &alias)
        }
        _ => {
            panic!("commands must be of valid strings!")
        }
    }
}
