//! Application graphics.

use anyhow::Result;
use minifb::Key;
use rugby::app;
use rugby::core::dmg::{self, Button};
use rugby::emu::part::joypad::Event;
use rugby::emu::part::video::Frame;
use rugby::extra::pal::Palette;

use self::win::{Main, Window};
#[cfg(feature = "gfx")]
use crate::app::dbg::gfx::Gfx;
use crate::exe::run::Cli;

pub mod win;

/// Application frontend.
#[derive(Debug)]
pub struct Frontend {
    /// Color palette.
    pub pal: Palette,
    /// Main window.
    pub lcd: Window<Main>,
    /// VRAM window group.
    #[cfg(feature = "gfx")]
    pub dbg: Gfx,
}

impl Frontend {
    /// Constructs a new `Graphics`.
    pub fn new(args: &Cli) -> Result<Self> {
        Ok(Self {
            pal: args.cfg.data.app.pal.clone().unwrap_or_default().into(),
            lcd: Window::open()?,
            #[cfg(feature = "gfx")]
            dbg: Gfx::default(),
        })
    }

    /// Checks if the frontend is alive.
    pub fn alive(&self) -> bool {
        self.lcd.is_open()
    }
}

impl app::joypad::Joypad for Frontend {
    type Button = Button;

    #[rustfmt::skip]
    fn events(&mut self) -> Vec<Event<Self::Button>> {
        self.lcd
            // Fetch keys
            .keys()
            .into_iter()
            // Perform key mapping
            .filter_map(|Event { input, state }| match input {
                Key::X         => Some(Event { input: Button::A,      state }),
                Key::Z         => Some(Event { input: Button::B,      state }),
                Key::Backspace => Some(Event { input: Button::Select, state }),
                Key::Enter     => Some(Event { input: Button::Start,  state }),
                Key::Right     => Some(Event { input: Button::Right,  state }),
                Key::Left      => Some(Event { input: Button::Left,   state }),
                Key::Up        => Some(Event { input: Button::Up,     state }),
                Key::Down      => Some(Event { input: Button::Down,   state }),
                _ => None,
            }).collect()
    }
}

impl app::video::Video for Frontend {
    type Pixel = dmg::ppu::Color;

    fn draw(&mut self, frame: Frame<Self::Pixel>) {
        // Translate pixels
        let frame = frame
            .iter()
            .map(|&pix| self.pal[pix as usize])
            .map(u32::from)
            .collect::<Vec<_>>();
        // Redraw main window
        self.lcd.redraw(&frame).unwrap();
    }
}
