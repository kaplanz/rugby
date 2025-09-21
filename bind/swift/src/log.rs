//! Logging API.

use std::ffi::{CString, c_char as char};

use log::{Metadata, Record};
pub use log::{debug, error, info, trace, warn};

/// External logger.
#[derive(Debug)]
struct Logger {
    /// Logging level filter.
    level: log::Level,
}

/// Global logger singleton.
static LOGGER: Logger = Logger {
    level: log::Level::Warn,
};

/// Initialize module.
pub(crate) fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}

unsafe extern "C" {
    /// Sends a log message to the caller.
    pub fn log(level: u64, target: *const char, message: *const char);
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LOGGER.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Prepare log message
            let lvl = record.level() as u64;
            let dir = CString::new(record.target()).unwrap();
            let msg = CString::new(record.args().to_string()).unwrap();
            // Forward log message
            unsafe { log(lvl, dir.as_ptr(), msg.as_ptr()) };
        }
    }

    fn flush(&self) {}
}

/// An enum representing the available verbosity levels of the logger.
type Level = log::Level;

/// Logging level.
#[uniffi::remote(Enum)]
enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    // This way these line up with the discriminants for LevelFilter below
    // This works because Rust treats field-less enums the same way as C does:
    // https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
    Error,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}
