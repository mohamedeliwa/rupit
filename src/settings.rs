use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// a list of all aliases sourced from the config file
    pub aliases: Value,
}

pub fn get_command_for_alias(alias: &str) -> Result<String, ConfigError> {
    let config_file_path = get_config_file_path();
    let settings: Settings = get_user_defined_settings(config_file_path).unwrap();

    let command = &settings.aliases[&alias];

    match command {
        Value::String(command) => Ok(command.to_string()),
        Value::Null => {
            panic!("\n{:?} alias not found in aliases list\n", &alias)
        }
        _ => {
            panic!("commands must be of valid strings!")
        }
    }
}

/// returns the path of the config file according the OS
///
/// as follows
///
/// Linux:   /home/\<user>/.config/rupit/rupit.json
///
/// Windows: C:\Users\\\<user>\AppData\Roaming\Foo Corp\Bar App\rupit.json
///
/// macOS:   /Users/\<user>/Library/Application Support/com.Foo-Corp.Bar-App/rupit.json
fn get_config_file_path() -> PathBuf {
    let config_dir = ProjectDirs::from("", "", "rupit").unwrap();
    let config_dir_path = config_dir.config_dir();
    config_dir_path.join("rupit.json")
}

fn get_user_defined_settings(config_file_path: PathBuf) -> Result<Settings, ConfigError> {
    // getting and parsing the user defined configuration file
    let settings = Config::builder()
        .set_default("aliases", "{}")?
        .add_source(File::from(config_file_path))
        // .add_source(File::new("rupit", FileFormat::Json))
        .build()
        .unwrap_or(
            Config::builder()
                .set_default("aliases", "{}")?
                .build()
                .unwrap(),
        );

    let settings: Settings = settings.try_deserialize::<Settings>().unwrap();
    Ok(settings)
}
