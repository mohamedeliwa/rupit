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
    // extracting the command alias from the cli arguments
    let args: Arguments = Arguments::parse();

    match args.command {
        Commands::Run(args) => {
            let alias = &args.alias;
            println!("\nalias: {:?}", alias);

            // getting the actual command from the config file
            let command = settings::get_command_for_alias(alias).unwrap();

            // executing the command
            runner::run_command(&command);
        }
        Commands::Show(args) => {
            println!("{:?}", args)
        }
    }
    Ok(())
}
