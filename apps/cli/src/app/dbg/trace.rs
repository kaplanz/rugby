//! Introspective tracing.

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Context;
use cmp::Tracer as Tracecmp;
use either::Either;
use log::Tracer as Tracelog;
use rugby::core::dmg::GameBoy;
use thiserror::Error;

use crate::exe::run::cli::trace::{Format, Trace};

/// Builds a tracing instance.
pub fn init(Trace { fmt, cmp, log }: &Trace) -> anyhow::Result<Tracer> {
    Ok(match (cmp, log) {
        (Some(cmp), None) => {
            Tracer::Cmp(Tracecmp {
                fmt: *fmt,
                src: BufReader::new(Box::new(if cmp == Path::new("-") {
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
        (None, log) => {
            Tracer::Log(Tracelog {
                fmt: *fmt,
                dst: BufWriter::new(Box::new(match log.as_deref() {
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
        _ => anyhow::bail!("unspecified mode for tracer"),
    })
}

/// Tracer instance.
pub enum Tracer {
    /// Tracing compare.
    Cmp(Tracecmp),
    /// Tracing logger.
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
    #[error(transparent)]
    Mismatch(#[from] cmp::Mismatch),
    /// Source log finished.
    #[error("finished comparison")]
    Finished,
}

/// Trace comparisons.
mod cmp {
    use std::fmt::Display;
    use std::io::{BufRead, BufReader, Read};
    use std::ops::Range;

    use annotate_snippets::{Level, Renderer, Snippet};
    use dissimilar::Chunk;
    use rugby::core::dmg;

    use super::{Error, Format, GameBoy};

    /// Comparing tracer.
    ///
    /// Compare against a file when tracing entries are emitted.
    pub struct Tracer {
        /// Tracing format.
        pub(super) fmt: Format,
        /// Trace compare file.
        pub(super) src: BufReader<Box<dyn Read>>,
        /// Trace line number.
        pub(super) idx: usize,
    }

    impl Tracer {
        /// Compare a trace entry against the provided tracelog file.
        pub fn emit(&mut self, emu: &GameBoy) -> super::Result<()> {
            // Read trace entry
            let mut buf = String::new();
            self.src.read_line(&mut buf)?;
            let expect = buf.trim_end();
            // Increment line number
            self.idx += 1;
            // Emit trace entry
            let actual = match self.fmt {
                Format::Binjgb => dmg::dbg::trace::binjgb,
                Format::Doctor => dmg::dbg::trace::doctor,
            }(emu);
            // Handle finished case
            if expect.is_empty() {
                return Err(Error::Finished);
            }
            // Compare trace entries
            self::diff(expect, &actual, self.idx).map_err(Into::into)
        }
    }

    /// Compares tracelog entries.
    fn diff(expect: &str, actual: &str, lineno: usize) -> Result<(), Mismatch> {
        // Quick identity check
        if expect == actual {
            return Ok(());
        }

        // Use itertools to find the first differing characters
        let mut count = 0;
        let chunks = dissimilar::diff(expect, actual)
            .iter()
            .filter_map(|change| match change {
                Chunk::Delete(s) => {
                    let index = count;
                    count += s.len();
                    Some(index..count)
                }
                Chunk::Insert(_) => None,
                Chunk::Equal(s) => {
                    count += s.len();
                    None
                }
            })
            .collect();

        // Create error with detailed information
        Err(Mismatch {
            lineno,
            expect: expect.to_string(),
            actual: actual.to_string(),
            chunks,
        })
    }
    /// Detailed tracelog mismatch report.
    #[derive(Debug)]
    pub struct Mismatch {
        lineno: usize,
        expect: String,
        actual: String,
        chunks: Vec<Range<usize>>,
    }

    impl Display for Mismatch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let notes = self
                .chunks
                .iter()
                .map(|diff| {
                    format!(
                        "expected `{}`, found `{}`",
                        &self.expect[diff.clone()],
                        &self.actual[diff.clone()],
                    )
                })
                .collect::<Vec<_>>();
            write!(
                f,
                "mismatch with tracelog entries\n\n{}",
                Renderer::styled().render(
                    Level::Note.title("see the following difference").snippet(
                        Snippet::source(&self.expect)
                            .line_start(self.lineno)
                            .annotations(self.chunks.iter().zip(notes.iter()).map(
                                |(diff, note)| { Level::Error.span(diff.clone()).label(note) }
                            ))
                    )
                )
            )
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
        /// Trace logging file.
        pub(super) dst: BufWriter<Box<dyn Write>>,
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
            writeln!(self.dst, "{entry}").map_err(Into::into)
        }
    }

    impl Write for Tracer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.dst.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.dst.flush()
        }
    }
}
