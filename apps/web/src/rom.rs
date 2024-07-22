use wasm_bindgen::prelude::*;

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
