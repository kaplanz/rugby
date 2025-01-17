//! Error types.

use thiserror::Error;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by an invalid operation.
#[derive(Clone, Debug, Error, uniffi::Error)]
#[error("{0}")]
pub enum Error {
    Message(String),
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Message(msg)
    }
}
