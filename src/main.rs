use clap::Parser;
use config::ConfigError;
use serde_json::Value;

mod runner;
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
                runner::run_command_windows(&command);
            } else {
                runner::run_command_unix(&command);
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
