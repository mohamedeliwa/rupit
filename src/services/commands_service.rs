use std::path::PathBuf;

use crate::Result;

pub trait Runner {
    fn run(&self, command: &str) -> Result<()>;
}

pub struct CommandsService<'a, T> {
    runner: &'a T,
}

impl<'a, T: Runner> CommandsService<'a, T> {
    pub fn new(runner: &'a T) -> Self {
        CommandsService { runner }
    }

    pub fn run_command(&self, command: &str) -> Result<()> {
        println!("\nexecuting command: {}...\n", command);
        self.runner.run(command)?;
        println!("\nRupit finished executing command: {}\n", command);
        Ok(())
    }

    pub fn print_config_file_path(&self, path: &PathBuf) -> Result<()> {
        println!("\n Rupit's config file path is:");
        println!("\n {:?}", path);
        Ok(())
    }
}
