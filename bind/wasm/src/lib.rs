//! WebAssembly bindings for `rugby`.
//!
//! This crate compiles to a `cdylib` and is packaged for WASM as `rugby-wasm`
//! via `[wasm-bindgen]`. It exposes the core emulator API to JavaScript on
//! browsers.
//!
//! A browser-compatible logging backend is initialised automatically when the
//! module loads.
//!
//! [wasm-bindgen]: https://wasm-bindgen.github.io/wasm-bindgen/

#![warn(clippy::pedantic)]

use wasm_bindgen::prelude::*;

mod api;

pub use self::api::*;

#[wasm_bindgen(start)]
fn start() {
    console_log::init().expect("error initializing logger");
}

/// Demo ROM.
#[wasm_bindgen]
#[must_use]
pub fn demo() -> Box<[u8]> {
    Box::from(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../roms/games/porklike/porklike.gb"
        ))
        .as_slice(),
    )
}
