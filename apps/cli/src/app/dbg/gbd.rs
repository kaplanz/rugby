//! Game Boy Debugger (GBD).

use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use log::{debug, error, trace};
use rugby::extra::gbd::prompt::{Error, Prompt};
use rustyline::DefaultEditor as Editor;
use rustyline::error::ReadlineError::{Eof, Interrupted as Int};
use rustyline::history::History;

use crate::dir;

/// Returns the path to the application's history file.
#[must_use]
pub fn history() -> PathBuf {
    dir::state().join("history")
}

/// Interface over the user's console.
#[derive(Debug)]
pub struct Console {
    /// Readline editor.
    edit: Editor,
    /// Show news on launch.
    news: bool,
}

impl Console {
    /// Constructs a new `Console`.
    #[expect(unused)]
    pub fn new() -> anyhow::Result<Self> {
        Self {
            edit: Editor::new()?,
            news: false,
        }
        .init()
    }

    /// Initializes the console.
    fn init(mut self) -> anyhow::Result<Self> {
        // Create state directory
        let state = dir::state();
        if !state.exists() {
            fs::create_dir_all(&state)
                .inspect(|()| trace!("created directory: `{}`", state.display()))
                .with_context(|| format!("failed to create: `{}`", state.display()))?;
        }
        // Set maximum history entries
        self.edit.history_mut().set_max_len(10_000)?;
        // Load previous history from file
        self.load()?;
        // Return initialized console
        Ok(self)
    }

    /// Loads history from a file into the prompt.
    fn load(&mut self) -> anyhow::Result<()> {
        // Get histfile path
        let path = self::history();
        if !path.exists() {
            // The likely hasn't used GBD before, so display help information
            // upon first launch.
            self.news = true;
            // Don't read any history if the file does not (yet) exist. Instead,
            // return without doing anything.
            return Ok(());
        }
        // Read history from file
        debug!("loading history: `{}`", path.display());
        self.edit
            .load_history(&path)
            .inspect(|()| {
                debug!(
                    "loaded history: {} items",
                    self.edit.history().iter().count()
                );
                trace!(
                    "history: {:#?}",
                    self.edit.history().iter().collect::<Vec<_>>()
                );
            })
            .context("error loading history")
    }

    /// Saves history from the prompt into a file.
    fn save(&mut self) -> anyhow::Result<()> {
        // Get histfile path
        let path = self::history();
        // Write history to file
        debug!("saving history: `{}`", path.display());
        self.edit
            .save_history(&path)
            .inspect(|()| {
                debug!(
                    "saved history: {} items",
                    self.edit.history().iter().count()
                );
                trace!(
                    "history: {:#?}",
                    self.edit.history().iter().collect::<Vec<_>>()
                );
            })
            .context("error saving history")
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        if let Err(err) = self.save() {
            error!("{err:#}");
        }
    }
}

impl Prompt for Console {
    fn prompt(&mut self, msg: &str) -> Result<String, Error> {
        // Show news on launch
        if std::mem::take(&mut self.news) {
            return Ok("help".to_string());
        }
        // Prompt the user for input
        let line = loop {
            match self.edit.readline(msg) {
                Ok(line) => break line,
                Err(err) => match err {
                    Int => (),
                    Eof => return Err(Error::Quit),
                    _ => return Err(Error::Internal(Box::new(err))),
                },
            }
        };
        // Add obtained input to history
        self.edit
            .history_mut()
            .add(&line)
            .map_err(|err| Box::new(err) as Box<dyn StdError>)?;
        // Return user input
        Ok(line)
    }
}
