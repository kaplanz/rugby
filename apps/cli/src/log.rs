use std::sync::OnceLock;

use anyhow::{Context, Result};
use clap_verbosity_flag::Verbosity;
use itertools::Itertools;
#[cfg(feature = "gbd")]
use rugby_gbd::Filter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, reload, EnvFilter, Layer, Registry};

type Reload = reload::Handle<EnvFilter, Registry>;

/// Global logger reload handle.
pub static RELOAD: OnceLock<Handle> = OnceLock::new();

/// Global logger verbosity.
pub static VERBOSE: OnceLock<Verbosity> = OnceLock::new();

/// Initializes the global logger.
///
/// # Note
///
/// Afterwards, the global logger's reload handle can be accessed via
/// [`RELOAD`].
pub fn init(filter: Option<&str>) -> Result<()> {
    // Build and configure an environment filter
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .parse({
            // Extract verbosity flag
            let verbose = VERBOSE.get().context("missing logging filter")?;
            // Get command-line verbosity
            let cli = verbose.is_present().then(|| verbose.to_string());
            // Get environment log filter
            let env = filter.map(ToString::to_string);
            // Combine supplied filters
            [env, cli].into_iter().flatten().join(",")
        })
        .with_context(|| format!("failed to parse: {filter:?}"))?;
    // Wrap it inside a reload layer
    let (filter, reload) = reload::Layer::new(filter);
    // Set global reload handle
    RELOAD
        .set(Handle::new(reload))
        // unable to set is an application error
        .expect("unable to set logger handle");
    // Install global logger
    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(filter))
        .try_init()
        .context("error installing logger")
}

/// Handle for reloading the logging filter.
#[cfg_attr(not(feature = "gbd"), allow(unused))]
#[derive(Clone, Debug)]
pub struct Handle {
    handle: Reload,
    filter: String,
}

impl Handle {
    /// Constructs a new `Handle` around a [`reload::Handle`].
    pub fn new(reload: Reload) -> Self {
        Self {
            filter: reload.with_current(ToString::to_string).unwrap(),
            handle: reload,
        }
    }
}

#[allow(unused)]
impl Handle {
    /// Inspect the logging filter.
    fn get(&self) -> &str {
        &self.filter
    }

    /// Changes the logging filter.
    fn set(&mut self, filter: String) {
        self.handle.reload(&filter).unwrap();
        self.filter = filter;
    }
}

#[cfg(feature = "gbd")]
impl Filter for Handle {
    fn get(&self) -> &str {
        Handle::get(self)
    }

    fn set(&mut self, filter: String) {
        Handle::set(self, filter);
    }
}
