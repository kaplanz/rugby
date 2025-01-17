//! Swift library port.
//!
//! Exported as a Swift package called `RugbyKit`.

uniffi::setup_scaffolding!();

mod api;
mod err;

pub use api::*;
pub use err::*;
