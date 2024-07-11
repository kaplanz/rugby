//! Introspective tracing.

use std::fs::File;
use std::io::{BufWriter, Write};

/// Tracing logfile.
///
/// Output handle where tracing entries are logged.
#[derive(Debug)]
pub struct Trace {
    buf: BufWriter<File>,
}

impl Trace {
    /// Constructs a new `Trace`.
    pub fn new(log: File) -> Self {
        Self {
            buf: BufWriter::new(log),
        }
    }
}

impl Write for Trace {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}
