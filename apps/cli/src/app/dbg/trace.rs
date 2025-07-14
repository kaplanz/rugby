//! Introspective tracing.

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, bail};
use cmp::Tracer as Tracecmp;
use either::Either;
use log::Tracer as Tracelog;
use rugby::core::dmg::GameBoy;
use thiserror::Error;

use crate::exe::run::cli::trace::{Format, Trace};

/// Builds a tracing instance.
pub fn init(Trace { fmt, log, cmp }: &Trace) -> anyhow::Result<Tracer> {
    Ok(match (log, cmp) {
        (None, Some(cmp)) => {
            Tracer::Cmp(Tracecmp {
                fmt: *fmt,
                log: BufReader::new(Box::new(if cmp == Path::new("-") {
                    // Use `stdin` for missing path or as alias of "-"
                    Either::Right(std::io::stdin())
                } else {
                    let path = cmp;
                    Either::Left(
                        File::open(path)
                            .with_context(|| format!("failed to open: `{}`", path.display()))?,
                    )
                })),
                idx: usize::default(),
            })
        }
        (log, None) => {
            Tracer::Log(Tracelog {
                fmt: *fmt,
                log: BufWriter::new(Box::new(match log.as_deref() {
                    // Make logfile at path
                    Some(path) if path != Path::new("-") => Either::Left({
                        File::create(path)
                            .with_context(|| format!("failed to open: `{}`", path.display()))?
                    }),
                    // Use `stdout` for missing path or as alias of "-"
                    _ => Either::Right(std::io::stdout()),
                })),
            })
        }
        _ => bail!("unspecified mode for tracer"),
    })
}

/// Tracer instance.
pub enum Tracer {
    /// Compare tracer.
    Cmp(Tracecmp),
    /// Logging tracer.
    Log(Tracelog),
}

/// Trace endpoint.
impl Tracer {
    /// Emit and handle a trace entry.
    pub fn emit(&mut self, emu: &GameBoy) -> Result<()> {
        match self {
            Tracer::Cmp(tracer) => tracer.emit(emu),
            Tracer::Log(tracer) => tracer.emit(emu),
        }
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by a tracelog operation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// I/O operation error.
    #[error(transparent)]
    Ioput(#[from] std::io::Error),
    /// Source log mismatch.
    #[error("tracelog entries did not match")]
    Mismatch(#[from] cmp::Mismatch),
    /// Source log finished.
    #[error("tracelog comparison has finished")]
    Finished,
}

/// Trace comparisons.
mod cmp {
    use std::fmt::Display;
    use std::io::{self, BufRead, BufReader, Read};

    use rugby::core::dmg;

    use super::{Error, Format, GameBoy};

    /// Compare tracer.
    ///
    /// Compares against a file as tracing entries are emitted.
    pub struct Tracer {
        /// Tracing format.
        pub(super) fmt: Format,
        /// Tracelog input file.
        pub(super) log: BufReader<Box<dyn Read>>,
        /// Trace line number.
        pub(super) idx: usize,
    }

    impl Tracer {
        /// Compare a trace entry against the provided tracelog file.
        pub fn emit(&mut self, emu: &GameBoy) -> super::Result<()> {
            let len = 256;

            // Read trace entry
            let expect = {
                // Read next line from the file
                let mut buf = String::with_capacity(len);
                let read = self.log.by_ref().take(256).read_line(&mut buf)?;
                // Ensure line isn't too long...
                if read == len && !buf.ends_with('\n') {
                    return Err(io::Error::new(
                        io::ErrorKind::FileTooLarge,
                        format!("line exceeded {len} byte limit"),
                    )
                    .into());
                }
                buf
            };
            let expect = expect.trim_end();
            // Increment line number
            self.idx += 1;

            // Emit trace entry
            let actual = match self.fmt {
                Format::Binjgb => dmg::dbg::trace::binjgb,
                Format::Doctor => dmg::dbg::trace::doctor,
            }(emu);
            let actual = actual.as_str();

            // Handle finished case
            if expect.is_empty() {
                return Err(Error::Finished);
            }

            // Perform equality check
            if expect == actual {
                // Success is when entries are equal
                Ok(())
            } else {
                // Compute rendered diff when unequal
                Err(Error::Mismatch(Mismatch {
                    line: self.idx,
                    diff: prettydiff::diff_chars(expect, actual).to_string(),
                }))
            }
        }
    }

    /// Tracelog mismatch report.
    #[derive(Debug)]
    pub struct Mismatch {
        line: usize,
        diff: String,
    }

    impl Display for Mismatch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "mismatch on line {}: `{}`", self.line, self.diff)
        }
    }

    impl std::error::Error for Mismatch {}
}

/// Tracelog writer.
mod log {
    use std::io::{BufWriter, Write};

    use rugby::core::dmg;

    use super::{Format, GameBoy};

    /// Logging tracer.
    ///
    /// Log to a file when tracing entries are emitted.
    pub struct Tracer {
        /// Tracing format.
        pub(super) fmt: Format,
        /// Tracelog output file.
        pub(super) log: BufWriter<Box<dyn Write>>,
    }

    impl Tracer {
        /// Emits a trace entry to the tracelog file.
        pub fn emit(&mut self, emu: &GameBoy) -> super::Result<()> {
            // Gather trace entry
            let entry = match self.fmt {
                Format::Binjgb => dmg::dbg::trace::binjgb,
                Format::Doctor => dmg::dbg::trace::doctor,
            }(emu);
            // Write to logfile
            writeln!(self.log, "{entry}").map_err(Into::into)
        }
    }
}
