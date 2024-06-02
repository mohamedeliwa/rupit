// use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Aliases = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationEntity {
    /// a list of all aliases sourced from the config file
    aliases: Aliases,
}

impl ConfigurationEntity {
    pub fn _new(aliases: Aliases) -> Self {
        ConfigurationEntity { aliases }
    }

    pub fn get_command_for_alias(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }
}
