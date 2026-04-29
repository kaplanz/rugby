//! Error types.

use rugby::core::cart;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by an invalid operation.
#[derive(Debug)]
#[derive(thiserror::Error)]
#[derive(uniffi::Error)]
#[error(transparent)]
#[uniffi(flat_error)]
pub enum Error {
    /// Input error.
    Ioput(#[from] std::io::Error),
    /// Header error.
    Header(#[from] cart::head::Error),
    /// Cartridge error.
    Cartridge(#[from] cart::Error),
}
