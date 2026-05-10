//! Cable API.

use std::io::{self, Read, Write};

/// Cable interface.
pub trait Cable {
    /// Receives data over the cable.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying read fails.
    fn recv(&mut self, buf: impl Read) -> io::Result<usize>;

    /// Sends data over the cable.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying write fails.
    fn send(&mut self, buf: impl Write) -> io::Result<usize>;
}
