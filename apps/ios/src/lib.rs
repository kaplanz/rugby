//! Swift library port.
//!
//! Exported as a Swift package called `RugbyKit`.

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
        log::init()
    });
}
