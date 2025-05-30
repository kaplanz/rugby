//! Game Boy Debugger (GBD).

#![warn(clippy::pedantic)]
// Allowed lints: clippy
#![allow(clippy::result_large_err)]

use std::fmt::{Debug, Display, Write};

use indexmap::IndexMap;
use log::debug;
use rugby_arch::Block;
use rugby_arch::reg::Port;
use rugby_core::dmg::{GameBoy, cpu, ppu};
use thiserror::Error;

use self::lang::Program;
use self::prompt::Prompt;

mod exec;
mod lang;

pub mod prompt;

pub use self::lang::{Command, Keyword};

/// Interactive debugger object.
#[derive(Debug, Default)]
pub struct Debugger {
    // Application
    cycle: usize,
    line: Option<Box<dyn Prompt>>,
    log: Option<Box<dyn Filter>>,
    // Console
    pc: u16,
    state: State,
    // Internal
    play: bool,
    freq: Tick,
    step: Option<usize>,
    prog: Option<Program>,
    prev: Option<Program>,
    bpts: IndexMap<u16, Option<Breakpoint>>,
}

impl Debugger {
    /// Constructs a new `Debugger` instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the logging handle.
    ///
    /// Used to change the logging level filter.
    pub fn logger(&mut self, log: impl Filter + 'static) {
        self.log = Some(Box::new(log));
    }

    /// Sets the prompt handle.
    ///
    /// Used to prompt the user for commands.
    pub fn prompt(&mut self, line: impl Prompt + 'static) {
        self.line = Some(Box::new(line));
    }

    /// Enables the debugger.
    pub fn enable(&mut self) {
        self.step = Some(0);
    }

    /// Resumes console emulation.
    pub fn resume(&mut self) {
        self.play = true;
    }

    /// Pauses console emulation.
    pub fn pause(&mut self) {
        self.play = false;
    }

    /// Checks if the console is paused.
    #[must_use]
    pub fn paused(&self) -> bool {
        !self.play
    }

    /// Synchronizes the debugger with the console.
    pub fn sync(&mut self, emu: &GameBoy) {
        let cpu = &emu.main.soc.cpu;
        let ppu = &emu.main.soc.ppu;

        // Update program counter
        self.pc = cpu.load(cpu::Select16::PC);
        self.state = State {
            cpu: cpu.stage().clone(),
            ppu: (ppu.dot(), ppu.load(ppu::Select::Ly)),
        };
    }

    /// Informs the user of the current emulation context.
    ///
    /// # Panics
    ///
    /// Cannot panic.
    pub fn inform(&self, emu: &GameBoy) {
        // Give context if recently paused
        if self.play {
            exec::list(self, emu).unwrap();
        }
    }

    /// Runs interactive debugger.
    ///
    /// # Errors
    ///
    /// Errors if the debugger failed to fetch, parse, or execute a command.
    pub fn run(&mut self, emu: &mut GameBoy) -> Result<()> {
        // Provide information to user before prompting for command
        self.inform(emu);
        // Prompt and execute commands until emulation resumed
        self.pause();
        'gbd: while self.paused() {
            let res = 'res: {
                // Attempt to fetch command
                let cmd = {
                    // Attempt to fetch the next command
                    if let cmd @ Some(_) = self.fetch() {
                        // It worked; use it
                        cmd
                    } else {
                        // Couldn't fetch; get program from user
                        match self.readline() {
                            // Program input; fetch next iteration
                            Ok(()) => continue 'gbd,
                            // No input; repeat previous program
                            Err(Error::Empty) => {
                                // Re-use previous program
                                self.prog.clone_from(&self.prev);
                                debug!("repeat program: `{:?}`", self.prog);
                                // Fetch command from repeated program
                                self.fetch()
                            }
                            // Prompt error; handle upstream
                            err @ Err(_) => {
                                // Clear previous program
                                self.prev = None;
                                // Raise prompt error upwards
                                break 'res err;
                            }
                        }
                    }
                };
                // Extract fetched command
                let Some(cmd) = cmd else {
                    // Command still not found; this case should
                    // only occur when no input has been provided,
                    // as otherwise the previously executed command
                    // should be repeated.
                    continue 'gbd;
                };
                // Execute fetched command
                self.exec(emu, cmd)
            };
            match res {
                Ok(()) => (),
                err @ Err(Error::Quit) => return err,
                Err(err) => advise::error!("{err}"),
            }
        }

        Ok(())
    }

    /// Prompts the user for a debugger program.
    ///
    /// # Errors
    ///
    /// Errors if the prompt failed.
    pub fn readline(&mut self) -> Result<()> {
        // Extract the prompt handle
        let line = self.line.as_mut().ok_or(Error::CfgPrompt)?;

        // Present the prompt; get input
        let fmt = format!("(#{} @ ${:04x})> ", self.cycle, self.pc);
        let input = match line.prompt(&fmt) {
            Err(prompt::Error::Quit) => return Err(Error::Quit),
            res => res?,
        };

        // Parse input
        let prog: Program = input.trim().parse()?;
        debug!("parsed program: `{prog:?}`");

        // Determine outcome
        if prog.is_empty() {
            // Remove stored program
            self.prog = None;
            // Report no input
            Err(Error::Empty)
        } else {
            // Store program; update previous
            self.prog = Some(prog);
            self.prev.clone_from(&self.prog);
            // Return successfully
            Ok(())
        }
    }

    /// Fetches the next command of the debugger program.
    pub fn fetch(&mut self) -> Option<Command> {
        self.prog.as_mut()?.pop_front()
    }

    /// Executes a debugger command.
    ///
    /// # Errors
    ///
    /// Errors if the command execution failed.
    ///
    /// # Panics
    ///
    /// Cannot panic.
    #[rustfmt::skip]
    pub fn exec(&mut self, emu: &mut GameBoy, cmd: Command) -> Result<()> {
        #[allow(clippy::enum_glob_use)]
        use Command::*;

        // Perform the command
        match cmd {
            Break(addr)             => exec::r#break(self, addr),
            Capture(path, force)    => exec::capture(emu, &path, force),
            Continue                => exec::r#continue(self, ),
            Delete(point)           => exec::delete(self, point),
            Disable(point)          => exec::disable(self, point),
            Enable(point)           => exec::enable(self, point),
            Freq(mode)              => exec::freq(self, mode),
            Goto(addr)              => exec::goto(emu, addr),
            Help(what)              => exec::help(what),
            Ignore(point, many)     => exec::ignore(self, point, many),
            Info(what)              => exec::info(self, what),
            Jump(addr)              => exec::jump(self, emu, addr),
            List                    => exec::list(self, emu),
            Load(loc)               => exec::loads(emu, loc),
            Log(filter)             => exec::log(self, filter),
            Quit                    => exec::quit(),
            Read(addr)              => exec::read(emu, addr),
            ReadRange(range)        => exec::read_range(emu, range),
            Reset                   => exec::reset(self, emu),
            Serial(mode)            => exec::serial(emu, mode),
            Step(many)              => exec::step(self, many),
            Store(loc, value)       => exec::stores(emu, loc, value),
            Write(addr, byte)       => exec::write(emu, addr, byte),
            WriteRange(range, byte) => exec::write_range(emu, range, byte),
        }
    }

    /// Returns whether the current cycle is an active edge cycle.
    ///
    /// Depending on the [`Debugger`]'s frequency setting, the definition of an
    /// edge may differ.
    #[rustfmt::skip]
    fn edge(&self) -> bool {
        // Pre-calculate machine cycle
        let (dx, ly) = self.state.ppu;
        let mtick = self.cycle % 4 == 0;
        let dinsn = matches!(self.state.cpu, cpu::Stage::Done);
        let dxrst = dx == 0;
        let lyrst = ly == 0;
        // Check if this is an edge cycle
        match self.freq {
            Tick::Dot   => true,
            Tick::Mach  => mtick,
            Tick::Insn  => mtick && dinsn,
            Tick::Line  => dxrst,
            Tick::Frame => dxrst && lyrst,
        }
    }
}

impl Block for Debugger {
    fn ready(&self) -> bool {
        // Is this an edge cycle?
        let edge = self.edge();
        // Is this cycle being stepped over?
        let step = self.step != Some(0);
        // Are we at a breakpoint?
        let bpt = self
            .bpts
            .get(&self.pc)
            .and_then(Option::as_ref)
            .is_some_and(|bpt| !bpt.disable && bpt.ignore == 0);
        // Should we enable the debugger?
        edge && (!step || bpt)
    }

    fn cycle(&mut self) {
        // Update application cycle count
        self.cycle += 1;
        // Yield on non-edge cycles
        if !self.edge() {
            return;
        }
        // Handle stepped over cycles
        if let Some(step) = &mut self.step {
            // Decrement step count
            *step = step.saturating_sub(1);
        }
        // Handle ignored breakpoints
        if let Some(bpt) = self.bpts.get_mut(&self.pc).and_then(Option::as_mut) {
            // Decrement ignore count
            bpt.ignore = bpt.ignore.saturating_sub(1);
        }
    }

    fn reset(&mut self) {
        // Application
        std::mem::take(&mut self.cycle);
    }
}

/// Debugging breakpoint metadata.
#[derive(Clone, Debug, Default)]
struct Breakpoint {
    disable: bool,
    ignore: usize,
}

impl Breakpoint {
    fn display(&self, point: usize, addr: u16) -> impl Display + use<> {
        let &Self { disable, ignore } = self;

        // Prepare format string
        let mut f = String::new();

        // Format the point, addr
        write!(f, "breakpoint {point} @ ${addr:04x}").unwrap();
        // Format characteristics
        if disable {
            write!(f, ": disabled").unwrap();
        } else if ignore > 0 {
            write!(f, ": will ignore next {ignore} crossings").unwrap();
        }

        f
    }
}

/// Debugger progress unit.
#[derive(Clone, Copy, Debug, Default)]
pub enum Tick {
    /// T-stage.
    ///
    /// True system clock notably used by the PPU; occurs at a frequency of 4
    /// MiHz.
    Dot,
    /// M-cycle.
    ///
    /// Basic clock for the CPU; always exactly 4 [dots][`Tick::Dot`].
    #[default]
    Mach,
    /// Instruction.
    ///
    /// Equal to the duration of the current instruction; always 1-6
    /// [M-cycles][`Tick::Mach`].
    Insn,
    /// Scanline.
    ///
    /// Duration to draw one line of the LCD display; always exactly 456
    /// [dots][`Tick::Dot`].
    Line,
    /// Frame.
    ///
    /// Duration to draw an entire frame of the LCD display; always exactly 154
    /// [scanlines][`Tick::Line`] (due to 10 extra v-blank scanlines).
    Frame,
}

#[rustfmt::skip]
impl Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dot   => "dot",
                Self::Mach  => "mach",
                Self::Insn  => "instruction",
                Self::Line  => "scanline",
                Self::Frame => "frame",
            }
        )
    }
}

/// Emulation state.
#[derive(Debug, Default)]
struct State {
    cpu: cpu::Stage,
    ppu: (u16, u8),
}

/// Handle for logger filter.
///
/// Provides a [getter](Self::get) and [setter](Self::set) to inspect and change
/// the logging filter.
pub trait Filter: Debug {
    /// Inspect the logging filter.
    fn get(&self) -> &str;

    /// Changes the logging filter.
    fn set(&mut self, filter: String);
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by a debugger command.
#[expect(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum Error {
    /// Requested breakpoint could not be found.
    #[error("breakpoint not found")]
    Breakpoint,
    /// Prompt has not been configured.
    #[error("prompt not configured")]
    CfgPrompt,
    /// Logger has not been configured.
    #[error("logger not configured")]
    CfgLogger,
    /// Prompt returned empty string.
    #[error("no input provided")]
    Empty,
    /// Image encoding error.
    #[error(transparent)]
    Image(#[from] png::EncodingError),
    /// I/O operation error.
    #[error(transparent)]
    Ioput(#[from] std::io::Error),
    /// Parsing returned an error.
    #[error(transparent)]
    Language(#[from] lang::Error),
    /// Prompt returned an error.
    #[error(transparent)]
    Prompt(#[from] prompt::Error),
    /// Quit requested by user.
    #[error("quit requested by user")]
    Quit,
    /// Attempted an unsupported operation.
    #[error("operation not supported")]
    Unsupported,
    /// Provided value does not match expectation.
    #[error("unexpected value")]
    Value,
}
