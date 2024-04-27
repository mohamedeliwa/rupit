use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Runs the corresponding command to the specified alias
    Run(RunArgs),
    /// Shows config data based on the args specified
    Show(ShowArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the alias for the command to execute
    pub alias: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ShowArgs {
    /// Prints the path of the config file
    #[arg(long, short)]
    pub config: bool,
    /// Prints the list of the available aliases in the config file
    #[arg(long, short)]
    pub aliases: bool,
}
