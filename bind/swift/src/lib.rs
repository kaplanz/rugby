//! Swift bindings for `rugby`.
//!
//! This crate compiles to a static library and packaged for Swift as `RugbyKit`
//! via [UniFFI]. It exposes the core emulator API to Swift on Apple platforms.
//!
//! The public surface is generated from UniFFI scaffolding. Call
//! [`initialize`](init) once before using any other API to set up the logging
//! backend.
//!
//! [UniFFI]: https://mozilla.github.io/uniffi-rs/

#![warn(clippy::pedantic)]
// Allowed lints: clippy
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

uniffi::setup_scaffolding!();

mod api;
mod err;
mod log;

use parking_lot::Once;

pub use self::api::*;
pub use self::err::*;
pub use self::log::*;

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
