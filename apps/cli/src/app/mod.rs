//! Application structures.

use std::time::Instant;

use anyhow::{Context as _, Result};
use log::debug;
use rugby::arch::Block;
#[cfg(feature = "win")]
use rugby::core::dmg;
#[cfg(feature = "log")]
use rugby::core::dmg::cpu::Stage;
use rugby::core::dmg::{Cartridge, GameBoy};
use rugby::prelude::*;
#[cfg(feature = "gbd")]
use rugby_gbd::Debugger;
use thiserror::Error;

use self::ctx::Context;
#[cfg(feature = "win")]
use self::gui::dbg::Region;
#[cfg(feature = "log")]
use crate::dbg::log::Tracer;
use crate::NAME;

mod ctx;

pub mod gui;

pub use self::gui::{Frontend, Graphics};

/// Clock divider.
///
/// As an optimization for more efficient synchronization, divide the target
/// frequency by this number, but clock this number as many cycles each time.
const DIVIDER: u32 = 0x100;

/// Signal notifier.
type Signal = std::sync::mpsc::Receiver<()>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Exit condition.
    pub bye: Option<Exit>,
    /// Application context.
    pub ctx: Option<Context>,
    /// Configuration data.
    pub cfg: Options,
    /// Emulator instance.
    pub emu: GameBoy,
    /// Graphical frontend.
    pub gui: Frontend,
    /// Signal handle.
    pub sig: Signal,
    /// Debug features.
    #[allow(unused)]
    #[cfg(feature = "debug")]
    pub dbg: Debug,
}

/// Exit condition.
#[derive(Debug, Error)]
pub enum Exit {
    /// Command-line.
    #[error("started with `-x/--exit`")]
    CommandLine,
    /// Debugger quit.
    #[cfg(feature = "gbd")]
    #[error("debugger quit")]
    Debugger,
    /// Graphical UI.
    #[error("window closed")]
    Graphics,
    /// Unix signals.
    #[error("interrupted")]
    Interrupt,
}

/// Configuration data.
#[derive(Debug)]
pub struct Options {
    /// Clock frequency.
    pub spd: Option<u32>,
}

/// Debug features.
#[cfg(feature = "debug")]
#[derive(Debug)]
pub struct Debug {
    /// Interactive debugger.
    #[cfg(feature = "gbd")]
    pub gbd: Option<Debugger>,
    /// Introspective tracing.
    #[cfg(feature = "log")]
    pub trace: Option<Tracer>,
    /// Graphical VRAM rendering.
    #[cfg(feature = "win")]
    pub win: bool,
}

impl App {
    /// Checks if the application is still running.
    pub fn done(&self) -> bool {
        self.bye
            .as_ref()
            .inspect(|err| debug!("exit reason: {err}"))
            .is_some()
    }

    /// Iteration of the main program.
    #[allow(clippy::too_many_lines)]
    pub fn main(&mut self) -> Result<()> {
        // Initialize context
        let Context {
            clock,
            count,
            timer,
        } = self.ctx.get_or_insert_with(|| Context::new(&self.cfg));

        // Break when GUI is closed
        if !self.gui.win.as_ref().map_or(true, Graphics::alive) {
            self.bye.get_or_insert(Exit::Graphics);
            return Ok(());
        }

        // Handle signals
        if self.sig.try_recv().is_ok() {
            debug!("received interrupt");

            // Trigger debugger
            #[cfg(feature = "gbd")]
            if let Some(gbd) = self.dbg.gbd.as_mut() {
                gbd.enable();
                return Ok(());
            }

            // Exit program
            self.bye.get_or_insert(Exit::Interrupt);
            return Ok(());
        }

        // Write trace entries.
        #[cfg(feature = "log")]
        if let Some(trace) = &mut self.dbg.trace {
            if matches!(self.emu.inside().proc().stage(), Stage::Fetch | Stage::Done)
                && count.delta % 4 == 0
            {
                trace
                    .log(&self.emu)
                    .context("failed to write trace entry")?;
            }
        }

        // Run debugger when enabled
        #[cfg(feature = "gbd")]
        if let Some(gbd) = self.dbg.gbd.as_mut() {
            // Sync with console
            gbd.sync(&self.emu);

            // Run debugger when enabled
            if gbd.ready() {
                let res = gbd.run(&mut self.emu, clock);
                // Quit if requested
                if matches!(res, Err(rugby_gbd::Error::Quit)) {
                    self.bye.get_or_insert(Exit::Debugger);
                    return Ok(());
                }
            }

            // Cycle debugger to remain synchronized with emulator
            gbd.cycle();
        }

        // Synchronize with wall-clock
        if count.cycle() % DIVIDER == 0 {
            // Delay until clock is ready
            clock.as_mut().map(Iterator::next);
        }

        // Emulate a single cycle
        self.emu.cycle();

        // Send joypad input
        if count.cycle() % 40 == 0 {
            // Fetch keys
            let keys = self.gui.input();
            // Update emulator
            self.emu.inside_mut().joypad().recv(keys);
        }

        // Sync serial data
        let serial = self.emu.inside_mut().serial();
        if count.cycle() % 0x10000 == 0 {
            // Receive data from emulator
            let rx = serial.rx();
            self.gui.recv(rx).context("failed to recv serial data")?;
            // Forward data to emulator
            let tx = serial.tx();
            self.gui.send(tx).context("failed to send serial data")?;
        }

        // Draw next frame
        let video = self.emu.inside().video();
        if video.vsync() {
            // Borrow frame
            let frame = video.frame();
            // Redraw screen
            self.gui.draw(frame);

            // Redraw debug windows
            #[cfg(feature = "win")]
            if self.dbg.win {
                if let Some(dbg) = self.gui.win.as_mut().map(|gui| &mut gui.dbg) {
                    // Gather debug info
                    let info = dmg::dbg::ppu(&mut self.emu);
                    // Extract PPU state
                    let recolor = |col: dmg::ppu::Color| self.gui.cfg.pal[col as usize].into();
                    let tdat = info.tdat.into_iter().map(recolor).collect::<Box<_>>();
                    let map1 = info.map1.into_iter().map(recolor).collect::<Box<_>>();
                    let map2 = info.map2.into_iter().map(recolor).collect::<Box<_>>();
                    // Display PPU state
                    dbg.get_mut(Region::Tile)
                        .map(|win| win.redraw(&tdat))
                        .transpose()
                        .context("failed to redraw tile data")?;
                    dbg.get_mut(Region::Map1)
                        .map(|win| win.redraw(&map1))
                        .transpose()
                        .context("failed to redraw tile map 1")?;
                    dbg.get_mut(Region::Map2)
                        .map(|win| win.redraw(&map2))
                        .transpose()
                        .context("failed to redraw tile map 2")?;
                }
            }
        }

        // Reset timing stats
        let elapsed = timer.elapsed();
        if elapsed.as_secs() > 0 {
            // Calculate stats
            let stats = count.stats(elapsed);
            debug!("{stats}");
            // Update GUI title
            if let Some(gui) = self.gui.win.as_mut() {
                gui.lcd.set_title(&format!(
                    "{title} ({frame:.1} FPS)",
                    title = self.emu.cart().map_or(NAME, Cartridge::title),
                    frame = stats.rate()
                ));
            }
            // Flush count, reset timer
            count.flush();
            *timer = Instant::now();
        } else {
            // Clock another cycle
            count.tick();
        }

        Ok(())
    }
}
