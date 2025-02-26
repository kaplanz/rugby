//! Application frontend.

use std::io::{self, Read, Write};

use log::{debug, trace};
use minifb::Key;
use rugby::app;
use rugby::core::dmg::{self, Button};
use rugby::emu::part::audio::Sample;
use rugby::emu::part::joypad::Event;
use rugby::emu::part::video::Frame;
use rugby::pal::Palette;

pub mod audio;
pub mod video;

pub use self::audio::Audio;
pub use self::video::Video;

/// Game link cable.
pub type Cable = std::net::UdpSocket;

/// Frontend options.
#[derive(Debug)]
pub struct Options {
    /// Color palette.
    pub pal: Palette,
}

/// Application frontend.
#[derive(Debug)]
pub struct Frontend {
    /// Frontend options.
    pub cfg: Options,
    /// Audio speaker.
    #[allow(unused)]
    pub aux: Option<Audio>,
    /// Game link cable.
    pub lnk: Option<Cable>,
    /// Video windows.
    pub win: Option<Video>,
}

impl app::audio::Audio for Frontend {
    fn play(&mut self, _: Sample) {
        todo!()
    }
}

impl app::joypad::Joypad for Frontend {
    type Button = Button;

    #[rustfmt::skip]
    fn events(&mut self) -> Vec<Event<Self::Button>> {
        self.win
            .as_ref()
            // Fetch keys
            .map(|gui| gui.lcd.keys())
            .into_iter()
            // Remove nested optional
            .flatten()
            // Perform key mapping
            .filter_map(|Event { input, state }| match input {
                Key::X     => Some(Event { input: Button::A,      state }),
                Key::Z     => Some(Event { input: Button::B,      state }),
                Key::Space => Some(Event { input: Button::Select, state }),
                Key::Enter => Some(Event { input: Button::Start,  state }),
                Key::Right => Some(Event { input: Button::Right,  state }),
                Key::Left  => Some(Event { input: Button::Left,   state }),
                Key::Up    => Some(Event { input: Button::Up,     state }),
                Key::Down  => Some(Event { input: Button::Down,   state }),
                _ => None,
            }).collect()
    }
}

impl app::serial::Serial for Frontend {
    fn recv(&mut self, mut tx: impl Read) -> io::Result<usize> {
        // Extract remote link
        let Some(link) = self.lnk.as_mut() else {
            return Ok(0);
        };
        // Download from emulator
        let mut buf = Vec::new();
        let read = tx.read_to_end(&mut buf)?;
        if read == 0 {
            return Ok(0);
        }
        // Transmit data to link
        let sent = link.send(&buf)?;
        debug!(
            "transmitted {sent} (downloaded {read})",
            sent = bfmt::Size::from(sent),
            read = bfmt::Size::from(read),
        );
        trace!("serial tx: {buf:?}");
        Ok(read)
    }

    fn send(&mut self, mut rx: impl Write) -> io::Result<usize> {
        // Extract remote link
        let Some(link) = self.lnk.as_mut() else {
            return Ok(0);
        };
        // Receive data from link
        let mut buf = [0; 0x10]; // use fixed-size buffer
        let recvd = match link.recv(&mut buf) {
            // Explicitly ignore would block error
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => Ok(0),
            res => res,
        }?;
        let buf = &buf[..recvd]; // retain only valid data
        if recvd == 0 {
            return Ok(0);
        }
        // Upload to emulator
        let wrote = rx.write(buf)?;
        debug!(
            "received {recvd} (uploaded {wrote})",
            recvd = bfmt::Size::from(recvd),
            wrote = bfmt::Size::from(wrote),
        );
        trace!("serial rx: {buf:?}");
        Ok(wrote)
    }
}

impl app::video::Video for Frontend {
    type Pixel = dmg::ppu::Color;

    fn draw(&mut self, frame: Frame<Self::Pixel>) {
        // Extract GUI
        let Some(gui) = self.win.as_mut() else {
            return;
        };
        // Translate pixels
        let frame = frame
            .iter()
            .map(|&pix| self.cfg.pal[pix as usize])
            .map(u32::from)
            .collect::<Vec<_>>();
        // Redraw main window
        gui.lcd.redraw(&frame).unwrap();
    }
}
