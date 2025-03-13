//! Swift library port.
//!
//! Exported as a Swift package called `RugbyKit`.

#![warn(clippy::pedantic)]
// Allowed lints: clippy
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

uniffi::setup_scaffolding!();

mod api;
mod err;
mod log;

use std::sync::Once;

pub use api::*;
pub use err::*;
pub use log::*;

/// Initialization singleton.
static INIT: Once = Once::new();

/// Initialize module.
#[uniffi::export(name = "initialize")]
pub fn init() {
    INIT.call_once(|| {
        // Initialize logging
        log::init();
    });
}
