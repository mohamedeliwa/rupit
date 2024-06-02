use std::path::PathBuf;

use config::{Config, File};
use directories::ProjectDirs;

use crate::{
    entities::ConfigurationEntity,
    services::{ConfigPathConstructor, ConfigurationParser},
    Result,
};

use super::Error;

pub struct ConfigurationAccess {
    application_name: String,
    config_file_name: String,
}

impl ConfigurationAccess {
    pub fn new(application_name: String) -> Self {
        ConfigurationAccess {
            config_file_name: format!("{}.json", application_name),
            application_name,
        }
    }
}

impl ConfigPathConstructor for ConfigurationAccess {
    /// returns the path of the config file according the OS
    ///
    /// as follows
    ///
    /// Linux:   /home/\<user>/.config/rupit/rupit.json
    ///
    /// Windows: C:\Users\\\<user>\AppData\Roaming\Foo Corp\Bar App\rupit.json
    ///
    /// macOS:   /Users/\<user>/Library/Application Support/com.Foo-Corp.Bar-App/rupit.json
    fn path_for_dir(&self) -> Option<std::path::PathBuf> {
        let config_dir = ProjectDirs::from("", "", &self.application_name)?;
        Some(config_dir.config_dir().to_path_buf())
    }

    fn path_for_file(&self) -> Option<PathBuf> {
        let config_dir_path = self.path_for_dir()?;
        let config_file_path = config_dir_path.join(&self.config_file_name);
        Some(config_file_path)
    }
}

impl ConfigurationParser for ConfigurationAccess {
    fn parse(&self) -> Result<ConfigurationEntity> {
        // getting and parsing the user defined configuration file
        let settings = Config::builder()
            .set_default("aliases", "{}")?
            .add_source(File::from(
                self.path_for_file().ok_or(Error::FaildGetFilePath)?,
            ))
            // .add_source(File::new("rupit", FileFormat::Json))
            .build()
            .unwrap_or(Config::builder().set_default("aliases", "{}")?.build()?);

        Ok(settings.try_deserialize::<ConfigurationEntity>()?)
    }
}
