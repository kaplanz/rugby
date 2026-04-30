//! Configuration groups.

mod audio;
mod boot;
mod cable;
mod cart;
mod input;
mod video;

pub use self::audio::Audio;
pub use self::boot::Boot;
pub use self::cable::Cable;
pub use self::cart::Cart;
pub use self::input::Input;
pub use self::video::{Palette, Video};
