// use crate::entities::Configuration;
use std::path::PathBuf;

use crate::entities::ConfigurationEntity;
use crate::Result;

pub trait ConfigPathConstructor {
    fn path_for_dir(&self) -> Option<PathBuf>;
    fn path_for_file(&self) -> Option<PathBuf>;
}

pub trait ConfigurationParser {
    fn parse(&self) -> Result<ConfigurationEntity>;
}

pub struct ConfigurationService<'a, T, E> {
    configuration_entity: Option<&'a ConfigurationEntity>,
    path_constructor: &'a T,
    configuration_parser: &'a E,
}

impl<'a, T: ConfigPathConstructor, E: ConfigurationParser> ConfigurationService<'a, T, E> {
    pub fn new(
        configuration_entity: Option<&'a ConfigurationEntity>,
        path_constructor: &'a T,
        configuration_parser: &'a E,
    ) -> Self {
        ConfigurationService {
            configuration_entity,
            path_constructor,
            configuration_parser,
        }
    }

    pub fn parse_configuration(&self) -> Result<ConfigurationEntity> {
        Ok(self.configuration_parser.parse()?)
    }

    pub fn register_configuration(&mut self, entity: &'a ConfigurationEntity) {
        self.configuration_entity = Some(entity);
    }

    pub fn _get_config_dir_path(&self) -> Option<PathBuf> {
        self.path_constructor.path_for_dir()
    }

    pub fn get_config_file_path(&self) -> Option<PathBuf> {
        self.path_constructor.path_for_file()
    }

    pub fn get_command_for_alias(&self, alias: &str) -> Option<&String> {
        match self.configuration_entity {
            Some(entity) => entity.get_command_for_alias(alias),
            None => None,
        }
    }
}
