//! Window implementation.

use std::collections::HashSet;
use std::marker::PhantomData;

pub use minifb::Key;
use minifb::{Result, Scale, ScaleMode, WindowOptions};
use rugby::emu::part::joypad::{Event, State};

/// Color value of an individual pixel.
pub type Pixel = u32;

/// Window attributes.
pub trait Attributes {
    /// Window title.
    const NAME: &str;

    /// Window frame.
    const SIZE: Extent;

    /// Window options.
    const OPTS: WindowOptions = WindowOptions {
        borderless: false,
        title: true,
        resize: true,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: false,
        transparency: false,
        none: false,
    };
}

/// Window frame.
///
/// Represents the logical size of a window.
#[derive(Debug)]
pub struct Extent {
    /// Width in pixels.
    pub wd: usize,
    /// Height in pixels.
    pub ht: usize,
}

impl From<(usize, usize)> for Extent {
    fn from((wd, ht): (usize, usize)) -> Self {
        Self { wd, ht }
    }
}

/// Application window.
#[derive(Debug)]
pub struct Window<A: Attributes> {
    /// Window handle.
    window: Box<minifb::Window>,
    /// Phantom data.
    _attrs: PhantomData<A>,
}

impl<A: Attributes> Window<A> {
    /// Opens a new `Window`.
    pub fn open() -> Result<Self> {
        minifb::Window::new(A::NAME, A::SIZE.wd, A::SIZE.ht, A::OPTS)
            .map(Box::new)
            .map(|window| Self {
                window,
                _attrs: PhantomData,
            })
            .map(Self::init)
    }

    /// Initializes a new `Window`.
    fn init(mut self) -> Self {
        self.update();
        self
    }

    /// Checks if the window is open.
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// Sets the window's title.
    pub fn title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    /// Redraws the window.
    pub fn redraw(&mut self, buf: &[Pixel]) -> Result<()> {
        self.window.update_with_buffer(buf, A::SIZE.wd, A::SIZE.ht)
    }

    /// Updates the window.
    pub fn update(&mut self) {
        self.window.update();
    }

    /// Collect window key events.
    pub fn keys(&self) -> Vec<Event<Key>> {
        // Get keys
        let dn: HashSet<_> = self
            .window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .into_iter()
            .collect();
        let up: HashSet<_> = self.window.get_keys_released().into_iter().collect();
        // Dedup keys
        let (dn, up) = (dn.difference(&up), up.difference(&dn));
        // Pair events
        let dn = dn.copied().map(|key| (key, State::Dn).into());
        let up = up.copied().map(|key| (key, State::Up).into());
        // Collect all
        dn.chain(up).collect()
    }
}
