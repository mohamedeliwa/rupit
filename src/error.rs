use config::ConfigError;
use derive_more::From;

use crate::{externals, services};

pub type Result<T> = std::result::Result<T, Error>;
// type Error = Box<dyn std::error::Error>;

#[derive(Debug, From)]
pub enum Error {
    // #[from]
    // Custom(String),

    // -- Modules
    #[from]
    Services(services::Error),
    #[from]
    Externals(externals::Error),

    // -- Externals
    #[from]
    Io(std::io::Error),

    #[from]
    ConfigError(ConfigError),
}

// region: --- Custom

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }

// impl From<&str> for Error {
//     fn from(value: &str) -> Self {
//         Self::Custom(value.to_string())
//     }
// }

// endregion: --- custom

// region: --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
        // write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
