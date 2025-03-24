//! Introspective tracing.

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};
use rugby::core::dmg::{self, GameBoy};

use crate::exe::run::cli::trace::{Format, Trace};

/// Builds a tracing instance.
pub fn init(Trace { fmt, log }: &Trace) -> Result<Tracer> {
    let log = match log.as_deref() {
        // Create a logfile from the path
        Some(path) if path != Path::new("-") => either::Either::Left({
            File::create(path).with_context(|| format!("failed to open: `{}`", path.display()))?
        }),
        // Use `stdout` missing path or as alias of "-"
        _ => either::Either::Right(std::io::stdout()),
    };
    Ok(Tracer::new(*fmt, log))
}

/// Tracing output.
///
/// Output handle where tracing entries are logged.
pub struct Tracer {
    /// Output format.
    pub fmt: Format,
    /// Trace logfile.
    log: BufWriter<Box<dyn Write>>,
}

impl Debug for Tracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("fmt", &self.fmt)
            .finish_non_exhaustive()
    }
}

impl Tracer {
    /// Constructs a new `Tracer`.
    pub fn new(fmt: Format, log: impl Write + 'static) -> Self {
        Self {
            log: BufWriter::new(Box::new(log)),
            fmt,
        }
    }

    /// Logs a single trace.
    pub fn log(&mut self, emu: &GameBoy) -> std::io::Result<()> {
        // Gather trace entry
        let entry = match self.fmt {
            Format::Binjgb => dmg::dbg::trace::binjgb,
            Format::Doctor => dmg::dbg::trace::doctor,
        }(emu);
        // Write to logfile
        writeln!(self.log, "{entry}")
    }
}

impl Write for Tracer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.log.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.log.flush()
    }
}
