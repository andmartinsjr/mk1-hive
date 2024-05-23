use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
}
