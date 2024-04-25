use clap::Parser;
use config::ConfigError;

mod runner;
mod settings;

#[derive(Parser, Debug)]
struct Arguments {
    /// the alias for the command/s to execute
    alias: String,
}

fn main() -> Result<(), ConfigError> {
    // extracting the command alias from the cli arguments
    let args: Arguments = Arguments::parse();
    let alias = &args.alias;
    println!("\nalias: {:?}", alias);

    // getting the actual command from the config file
    let command = settings::get_command_for_alias(alias).unwrap();

    // executing the command
    runner::run_command(&command);

    Ok(())
}
