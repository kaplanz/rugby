//! Error types.

use std::error::Error as StdError;
use std::fmt::Display;
use std::process::{ExitCode, Termination};

use advise::Render;
use clap::builder::styling::{AnsiColor, Style};
use thiserror::Error;

use crate::cfg;

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A top-level error from within the application.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Application error.
    #[error(transparent)]
    App(#[from] anyhow::Error),
    /// Configuration error.
    #[error(transparent)]
    Cfg(#[from] cfg::Error),
}

impl Error {
    /// Advises the user about an error.
    fn advise(&self) {
        // Report top-level error
        advise::error!("{}", format!("{self}").trim_end());
        let Some(mut err) = self.source() else {
            return;
        };
        // Report intermediate errors
        while let Some(src) = err.source() {
            advise::advise!(Cause::Stem, "{}", format!("{err}").trim_end());
            err = src;
        }
        // Report bottom-level error
        advise::advise!(Cause::Root, "{}", format!("{err}").trim_end());
    }
}

impl From<Error> for ExitCode {
    fn from(err: Error) -> Self {
        match err {
            Error::App(_) => ExitCode::FAILURE,
            Error::Cfg(_) => sysexits::ExitCode::Config.into(),
        }
    }
}

/// Application exit condition.
///
/// In the [`Termination`] implementation for `Exit`, we print any errors that
/// occur for the user.
#[derive(Debug)]
pub enum Exit {
    /// Exit success.
    Success,
    /// Exit failure.
    ///
    /// Advises the user about the [error](enum@Error), returning a non-zero
    /// [exit code](ExitCode).
    Failure(Error),
}

impl<E: Into<Error>> From<E> for Exit {
    fn from(err: E) -> Self {
        Self::Failure(err.into())
    }
}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        match self {
            Exit::Success => {
                // Return a success exit code
                ExitCode::SUCCESS
            }
            Exit::Failure(err) => {
                // Advise the user about the error
                err.advise();
                // Return a failure exit code
                err.into()
            }
        }
    }
}

/// Rendering for error reporting.
#[derive(Debug)]
enum Cause {
    /// Intermediate error source.
    Stem,
    /// Bottom-level error cause.
    Root,
}

impl Render for Cause {
    fn style(&self) -> Style {
        AnsiColor::Red.on_default()
    }

    fn label(&self) -> impl Display {
        match self {
            Cause::Stem => "   ├─",
            Cause::Root => "   ╰─",
        }
    }
}
