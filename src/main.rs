use clap::Parser;

mod arguments;
use arguments::{Arguments, Commands};
mod entities;
mod error;
use error::{Error, Result};
mod externals;
mod services;

fn main() -> Result<()> {
    let args: Arguments = Arguments::parse();
    let app_name = String::from("rupit");

    // region: initiating entities and injecting dependencies
    let configuration_access = externals::ConfigurationAccess::new(app_name);
    let mut configuration_service =
        services::ConfigurationService::new(None, &configuration_access, &configuration_access);
    let configuration_entity = configuration_service.parse_configuration()?;
    configuration_service.register_configuration(&configuration_entity);
    let command_runner = externals::CommandRuner {};
    let command_service = services::CommandsService::new(&command_runner);
    // endregion: initiating entities and injecting dependencies

    match args.command {
        Commands::Run(args) => {
            let alias = &args.alias;
            println!("\nalias: {}", alias);

            let command = configuration_service
                .get_command_for_alias(alias)
                .ok_or(Error::Services(services::Error::AliasNotFound))?;

            command_service.run_command(&command)?;
        }
        Commands::Show(args) => {
            if args.config {
                let config_file_path = configuration_service
                    .get_config_file_path()
                    .ok_or(Error::Services(services::Error::FailedToGetFilePath))?;

                command_service.print_config_file_path(&config_file_path)?;
            } else if args.alias.is_some() {
                let command = configuration_service
                    .get_command_for_alias(
                        &args
                            .alias
                            .as_ref()
                            .expect("a value of alias must always exist, ie can't be None"),
                    )
                    .ok_or(Error::Services(services::Error::AliasNotFound))?;

                println!("\n the command is:");
                println!("\n {}", command);
            }
        }
    }
    Ok(())
}
