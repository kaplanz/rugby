use rugby::arch::Block;
use rugby::core::dmg;
use rugby::prelude::*;
use wasm_bindgen::prelude::*;

pub mod cart;

/// Game Boy model.
#[derive(Debug, Default)]
#[wasm_bindgen(inspectable)]
pub struct GameBoy(pub(crate) dmg::GameBoy);

#[wasm_bindgen]
impl GameBoy {
    /// Constructs a new `GameBoy`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(dmg::GameBoy::new())
    }
}

#[wasm_bindgen]
impl GameBoy {
    /// Checks if enabled.
    pub fn ready(&self) -> bool {
        self.0.ready()
    }

    /// Emulates a single cycle.
    pub fn cycle(&mut self) {
        self.0.cycle();
    }

    /// Performs a reset.
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[wasm_bindgen]
impl GameBoy {
    /// Checks if the frame is ready to be rendered.
    pub fn vsync(&self) -> bool {
        self.0.inside().video().vsync()
    }

    /// Retrieves the current frame state.
    pub fn frame(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(
            self.0
                .inside()
                .video()
                .frame()
                .iter()
                .map(|&pix| pix as u8)
                .collect::<Box<[u8]>>()
                .as_ref(),
        )
    }
}
