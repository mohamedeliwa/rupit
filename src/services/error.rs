use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    // -- configuration_service
    AliasNotFound,
    FailedToGetFilePath,
}
