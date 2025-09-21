//! Error types.

use rugby::core::dmg;
use thiserror::Error;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by an invalid operation.
#[derive(Debug, Error, uniffi::Error)]
#[error(transparent)]
#[uniffi(flat_error)]
pub enum Error {
    /// Input error.
    Ioput(#[from] std::io::Error),
    /// Header error.
    Header(#[from] dmg::cart::head::Error),
    /// Cartridge error.
    Cartridge(#[from] dmg::cart::Error),
}
