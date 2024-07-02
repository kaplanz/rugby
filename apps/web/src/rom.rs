use wasm_bindgen::prelude::*;

/// `dmg-acid2` Test ROM.
#[wasm_bindgen]
pub fn acid2() -> Box<[u8]> {
    Box::from(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../roms/test/acid2/dmg-acid2.gb"
        ))
        .as_slice(),
    )
}
