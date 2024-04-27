use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    /// a list of all aliases sourced from the config file
    aliases: Value,
}

pub fn get_command_for_alias(alias: &str) -> Result<String, ConfigError> {
    let config_file_path = get_config_file_path().ok_or(ConfigError::Message(String::from(
        "couldn't get config file path from the OS",
    )))?;

    let settings: Settings = get_user_defined_settings(config_file_path)?;

    let command = &settings.aliases[&alias];

    match command {
        Value::String(command) => Ok(command.to_string()),
        Value::Null => Err(ConfigError::Message(format!(
            "{:?} alias not found in aliases list",
            &alias
        ))),
        _ => Err(ConfigError::Message(String::from(
            "command must be a valid string!",
        ))),
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
pub fn get_config_file_path() -> Option<PathBuf> {
    let config_dir = ProjectDirs::from("", "", "rupit")?;
    let config_file_path = config_dir.config_dir().join("rupit.json");
    Some(config_file_path)
}

fn get_user_defined_settings(config_file_path: PathBuf) -> Result<Settings, ConfigError> {
    // getting and parsing the user defined configuration file
    let settings = Config::builder()
        .set_default("aliases", "{}")?
        .add_source(File::from(config_file_path))
        // .add_source(File::new("rupit", FileFormat::Json))
        .build()
        .unwrap_or(Config::builder().set_default("aliases", "{}")?.build()?);

    settings.try_deserialize::<Settings>()
}
