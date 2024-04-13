use clap::Parser;
use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    // extracting the command alias from the cli arguments
    let args: Arguments = Arguments::parse();

    // getting the list of aliases from the config file
    let settings = Config::builder()
        .set_default("aliases", "{}")?
        .add_source(File::new("rupit", FileFormat::Json))
        .build()
        .unwrap();

    let settings: Settings = settings.try_deserialize::<Settings>().unwrap();

    // getting the actual command from the config file
    let alias = args.alias;
    let command = settings.aliases[&alias].clone();

    match command {
        Value::String(command) => {
            println!("alias: {:?}", alias);
            println!("\nrunning command: {}...\n", command);

            if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(["/C", &command])
                    .status()
                    .expect(&format!("{} command failed to execute", &command));
            } else {
                std::process::Command::new("sh")
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
