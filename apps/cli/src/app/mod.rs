//! Application runtime.

use self::ctrl::{Exit, exit, reason, running};

#[cfg(feature = "debug")]
mod dbg;
mod gui;
mod run;

pub use self::run::main as run;

/// Application assembly.
pub mod init {
    pub use super::run::emu::init::*;
}

/// Application teardown.
pub mod drop {
    pub use super::run::emu::drop::*;
}

/// Application persists.
pub mod save {
    pub use super::run::emu::save as ram;
}

/// Application statics.
pub mod data {
    /// Audio state.
    pub mod audio {
        use std::sync::LazyLock;

        use parking_lot::Mutex;
        use rugby::emu::part::audio::Sample;

        use crate::app::run::aux::Stream;

        /// Thread-safe shared audio stream.
        static STREAM: LazyLock<Mutex<Stream>> = LazyLock::new(|| Mutex::new(Stream::new()));

        /// Push a sample to the audio system.
        ///
        /// This function blocks if the mutex is held by another thread.
        pub fn push(sample: Sample) {
            STREAM.lock().push(sample);
        }

        /// Pull a sample from the audio system.
        ///
        /// This function blocks if the mutex is held by another thread.
        pub fn pull() -> Option<Sample> {
            STREAM.lock().pull()
        }
    }

    /// Benchmarking.
    pub mod bench {
        use parking_lot::Mutex;

        /// Benchmark report.
        static REPORT: Mutex<Option<f64>> = Mutex::new(None);

        /// Updates the most recent performance.
        ///
        /// This function blocks if the mutex is held by another thread.
        pub fn update(freq: f64) {
            REPORT.lock().replace(freq);
        }

        /// Reports the most recent performance.
        ///
        /// This function will never block.
        pub fn report() -> Option<f64> {
            REPORT.try_lock()?.take()
        }
    }

    /// Debug state.
    #[cfg(feature = "debug")]
    pub mod debug {
        /// Graphics debug windows.
        #[cfg(feature = "gfx")]
        pub mod gfx {
            use parking_lot::Mutex;
            use rugby::core::dmg::ppu;

            /// Debug graphics.
            static FRAME: Mutex<Option<ppu::dbg::Debug>> = Mutex::new(None);

            /// Write the most recent debug frame for drawing.
            ///
            /// This function blocks if the mutex is held by another thread.
            pub fn draw(frame: ppu::dbg::Debug) {
                FRAME.lock().replace(frame);
            }

            /// Takes the most recent debug frame for drawing.
            ///
            /// This function will never block.
            pub fn take() -> Option<ppu::dbg::Debug> {
                FRAME.try_lock()?.take()
            }
        }
    }

    /// Input state.
    pub mod input {
        use parking_lot::Mutex;
        use rugby::core::dmg::Button;
        use rugby::emu::part::joypad::Event;

        /// Input queue.
        static INPUT: Mutex<Vec<Event<Button>>> = Mutex::new(Vec::new());

        /// Sends key events to input queue.
        ///
        /// This function blocks if the mutex is held by another thread.
        pub fn send(mut keys: Vec<Event<Button>>) {
            INPUT.lock().append(&mut keys);
        }

        /// Takes all queued key events.
        ///
        /// This function will never block.
        pub fn take() -> Vec<Event<Button>> {
            INPUT
                .try_lock()
                .map_or_else(Vec::default, |mut keys| std::mem::take(&mut keys))
        }
    }

    /// Video state.
    pub mod video {
        use parking_lot::Mutex;
        use rugby::core::dmg::ppu::Frame;

        /// Video framebuffer.
        static FRAME: Mutex<Option<Frame>> = Mutex::new(None);

        /// Write the most recent frame for drawing.
        ///
        /// This function blocks if the mutex is held by another thread.
        pub fn draw(frame: Frame) {
            FRAME.lock().replace(frame);
        }

        /// Takes the most recent frame for drawing.
        ///
        /// This function will never block.
        pub fn take() -> Option<Frame> {
            FRAME.try_lock()?.take()
        }
    }
}

/// Application control.
mod ctrl {
    use std::sync::atomic::{AtomicU8, Ordering};

    use num_enum::TryFromPrimitive;
    use thiserror::Error;

    /// Exit condition.
    #[derive(Debug, Error, TryFromPrimitive)]
    #[repr(u8)]
    pub enum Exit {
        /// Unknown exit reason.
        #[error("unknown exit case")]
        Unknown = 1,
        /// Runtime error occurred.
        #[error("got runtime error")]
        Error,
        /// Passed `--exit` flag.
        #[error("command-line flag")]
        CliFlag,
        /// Quit via debugger.
        #[cfg(feature = "gbd")]
        #[error("quit via debugger")]
        Debugger,
        /// Application closed.
        #[error("app window closed")]
        Frontend,
        /// Interrupt signal.
        #[error("interrupt by user")]
        Interrupt,
    }

    /// Application exit flag.
    ///
    /// This value, `false` at initialization, will change to `true` exactly
    /// once during the lifetime of the program, signaling to all threads that
    /// they should exit.
    static EXIT: AtomicU8 = AtomicU8::new(0);

    /// Polls if the application is still running.
    ///
    /// # Performance
    ///
    /// This function is optimized to run as efficiently as possible. As such,
    /// it should be performant enough to run in a hot-loop without being a
    /// bottleneck.
    pub fn running() -> bool {
        EXIT.load(Ordering::Relaxed) == 0
    }

    /// Returns the exit reason.
    ///
    /// If the application is still [`running`], this will return `None`.
    pub fn reason() -> Option<Exit> {
        Exit::try_from(EXIT.load(Ordering::Relaxed)).ok()
    }

    /// Signals all threads to exit using the supplied reason.
    ///
    /// This doesn't directly send terminate threads, rather, it simply changes
    /// the result obtained when [polling](running). If the exit reason has
    /// already been set, this is a no-op. (Does not overwrite existing reason.)
    pub fn exit(reason: Exit) {
        let _ = EXIT.compare_exchange(0, reason as u8, Ordering::Relaxed, Ordering::Relaxed);
    }
}

/// Application utilities.
mod util {
    use std::ffi::OsStr;
    use std::path::Path;

    use rugby::extra::cfg::opt;

    /// Resolves the application title.
    pub fn title(args: &opt::emu::Cart) -> &str {
        args.rom
            .as_deref()
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .unwrap_or(crate::NAME)
    }
}
