//! Frontend thread.

use std::sync::mpsc;

use anyhow::{Context as _, Result};
use log::{debug, error, info, trace};
use rugby::prelude::*;
use thiserror::Error;

use crate::exe::run::Cli;
use crate::gui::Frontend;
use crate::talk::{self, Channel};
use crate::{emu, init, util};

pub mod msg;

pub use self::msg::Message;

/// Signal notifier.
type Signal = mpsc::Receiver<()>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Exit condition.
    pub bye: Option<Exit>,
    /// Application frontend.
    pub gui: Frontend,
    /// Signal handle.
    pub sig: Signal,
}

/// Exit condition.
#[derive(Debug, Error)]
pub enum Exit {
    /// Passed `--exit` flag.
    #[error("passed `--exit` flag")]
    CommandLine,
    /// Quit via debugger.
    #[cfg(feature = "gbd")]
    #[error("quit via debugger")]
    Debugger,
    /// Application closed.
    #[error("application closed")]
    Graphics,
    /// Interrupt signal.
    #[error("interrupt signal")]
    Interrupt,
}

impl App {
    /// Checks whether the application should exit.
    pub fn done(&self) -> bool {
        // Return exit status
        self.bye
            .as_ref()
            .inspect(|err| info!("exit: {err}"))
            .is_some()
    }
}

/// Frontend main.
pub fn main(args: &Cli, mut talk: Channel<Message, emu::Message>) -> Result<()> {
    // Instantiate frontend
    let mut app = init::app(args)?;
    // Start emulator
    if app.bye.is_none() {
        talk.send(emu::Message::Play)?;
    }
    // Extract title
    let title = util::title(&args.cfg.data.emu.cart);

    // Frontend loop
    let mut res = (|| -> Result<()> {
        loop {
            // Check for termination
            if app.done() {
                // Exit emulator
                talk.send(emu::Message::Exit)?;
                // Exit frontend
                return Ok(());
            }

            // Poll key events
            let keys = app.gui.input();
            if !keys.is_empty() {
                debug!("keys: {keys:?}");
                talk.send(emu::Message::Data(emu::msg::Data::Joypad(keys)))?;
            }

            // Read messages
            let msg = talk.recv()?;
            // Process messages
            if let Some(msg) = msg {
                trace!("recv: {msg}");
                match msg {
                    #[cfg(feature = "win")]
                    Message::Debug(msg::Debug::Vram(info)) => {
                        // Render VRAM debug info
                        app.gui.vram(info).context("error rendering VRAM")?;
                    }
                    Message::Stats(stats) => {
                        // Update title with statistics
                        if let Some(gui) = app.gui.win.as_mut() {
                            gui.lcd
                                .title(&format!("{title} ({frame:.1} FPS)", frame = stats.rate()));
                        }
                    }
                    Message::Video(frame) => {
                        // Draw next frame
                        app.gui.draw(&frame);
                        // Send acknowledgment
                        talk.send(emu::Message::Sync(emu::msg::Sync::Video))?;
                    }
                    Message::Exit(exit) => {
                        // Set exit condition
                        app.bye.get_or_insert(exit);
                    }
                }
                // Process next message
                continue;
            }

            // Update graphics
            if let Some(gui) = app.gui.win.as_mut() {
                // Check liveness
                if gui.alive() {
                    // Refresh window
                    gui.lcd.update();
                } else {
                    // Exit if closed
                    app.bye.get_or_insert(Exit::Graphics);
                    continue;
                }
            }
            // Handle signals
            if app.sig.try_recv().is_ok() {
                // Trigger debugger if enabled
                #[cfg(feature = "gbd")]
                if args.dbg.gbd {
                    debug!("pausing debugger...");
                    talk.send(emu::Message::Break)?;
                    continue; // don't fallthrough
                }
                // Exit program otherwise
                debug!("exiting emulator...");
                app.bye.get_or_insert(Exit::Interrupt);
                continue;
            }
        }
    })(); // NOTE: This weird syntax is in lieu of using unstable try blocks.

    // Inspect error
    if let Some(err) = res
        .as_ref()
        .err()
        .and_then(|err| err.downcast_ref::<talk::Error>())
    {
        // Ignore disconnect
        debug!("{err}");
        res = Ok(());
    } else {
        // Exit emulator
        talk.send(emu::Message::Exit)?;
    }
    // Propagate errors
    res.context("frontend error occurred")
}
