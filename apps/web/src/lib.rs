use wasm_bindgen::prelude::*;

pub mod emu;
pub mod key;
pub mod rom;

#[wasm_bindgen(start)]
fn start() {
    console_log::init().expect("error initializing logger");
}
