//! WebAssembly library port.
//!
//! Exported as a JS package called `rugby-web`.

use wasm_bindgen::prelude::*;

mod api;

pub use api::*;

#[wasm_bindgen(start)]
fn start() {
    console_log::init().expect("error initializing logger");
}

/// Demo ROM.
#[wasm_bindgen]
pub fn demo() -> Box<[u8]> {
    Box::from(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../roms/games/porklike/porklike.gb"
        ))
        .as_slice(),
    )
}
