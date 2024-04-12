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

    println!("alias: {:?}", alias);
    println!("command: {:?}", command);

    // executing the command
    todo!();

    Ok(())
}
