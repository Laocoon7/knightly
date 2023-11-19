use ron::error::SpannedError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Encountered an io error while loading asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Encountered a deserialization error while loading asset: {0}")]
    Ron(#[from] SpannedError),
}
