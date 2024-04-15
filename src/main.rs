use clap::Parser;
use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, ffi::OsString};

#[derive(Parser, Debug)]
struct Arguments {
    /// the alias for the command/s to execute
    alias: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    /// a list of all aliases sourced from the config file
    aliases: Value,
}

fn main() -> Result<(), ConfigError> {
    // getting the path of the config file
    // Linux:   /home/<user>/.config/rupit
    // Windows: C:\Users\<user>\AppData\Roaming\Foo Corp\Bar App
    // macOS:   /Users/<user>/Library/Application Support/com.Foo-Corp.Bar-App
    let config_dir = ProjectDirs::from("", "", "rupit").unwrap();
    let config_dir_path = config_dir.config_dir();
    let config_file_path = config_dir_path.join("rupit.json");

    // extracting the command alias from the cli arguments
    let args: Arguments = Arguments::parse();

    // getting the list of aliases from the config file
    let settings = Config::builder()
        .set_default("aliases", "{}")?
        .add_source(File::from(config_file_path))
        // .add_source(File::new("rupit", FileFormat::Json))
        .build()
        .unwrap_or(
            Config::builder()
                .set_default("aliases", "{}")?
                .build()
                .unwrap(),
        );

    let settings: Settings = settings.try_deserialize::<Settings>().unwrap();

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
