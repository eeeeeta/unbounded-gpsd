//! Result and Error Types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpsdError {
    #[error("gpsd connection closed")]
    GpsdFailed,
    #[error("failed to deserialize text '{0}': {1}")]
    DeserFailed(String, serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type GpsdResult<T> = Result<T, GpsdError>;
