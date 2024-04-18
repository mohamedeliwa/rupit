use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    /// a list of all aliases sourced from the config file
    pub aliases: Value,
}
