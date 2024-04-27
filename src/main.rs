use clap::{Args, Parser, Subcommand};
use config::ConfigError;

mod runner;
mod settings;

#[derive(Parser, Debug)]
#[command(version, propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs the corresponding command to the specified alias
    Run(RunArgs),
    /// Shows config data based on the args specified
    Show(ShowArgs),
}

#[derive(Args, Debug)]
struct RunArgs {
    /// the alias for the command to execute
    alias: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct ShowArgs {
    /// Prints the path of the config file
    #[arg(long, short)]
    config: bool,
    /// Prints the list of the available aliases in the config file
    #[arg(long, short)]
    aliases: bool,
}

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
