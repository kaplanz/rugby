//! Gameboy Doctor.

use std::fs::File;
use std::io::{BufWriter, Write};

/// Doctor logfile.
///
/// Logging output destination for CPU introspection formatted as specified for
/// [Gameboy Doctor][gbdoc].
///
/// [gbdoc]: https://robertheaton.com/gameboy-doctor
#[derive(Debug)]
pub struct Doctor {
    buf: BufWriter<File>,
}

impl Doctor {
    pub fn new(log: File) -> Self {
        Self {
            buf: BufWriter::new(log),
        }
    }
}

impl Write for Doctor {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}
