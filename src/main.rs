use clap::Parser;
use config::ConfigError;

mod arguments;
use arguments::{Arguments, Commands};
mod runner;
mod settings;

fn main() -> Result<(), ConfigError> {
    let args: Arguments = Arguments::parse();

    match args.command {
        Commands::Run(args) => {
            let alias = &args.alias;
            println!("\nalias: {:?}", alias);

            let command = settings::get_command_for_alias(alias)?;

            runner::run_command(&command);
        }
        Commands::Show(args) => {
            if args.config {
                let config_file_path = settings::get_config_file_path().ok_or(
                    ConfigError::Message(String::from("couldn't get config file path from the OS")),
                )?;

                println!("\n Rupit's config file path is:");
                println!("\n {:?}", config_file_path);
            } else if args.aliases {
                println!("\n Listing available aliases is not supported yet!");
            }
        }
    }
    Ok(())
}
