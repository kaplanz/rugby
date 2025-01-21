//! Emulator configuration.

use std::path::Path;

use crate::opt::{Emulator, Frontend};

mod val;

pub mod env;
pub mod opt;

/// Configuration interface.
pub trait Join {
    /// Rebase relative paths to the provided root.
    ///
    /// Any relative paths will have be rebased such that they are not relative
    /// to the provided root.
    fn rebase(&mut self, root: &Path);

    /// Combines two configuration object instances.
    ///
    /// This is useful when some configurations may also be supplied on the
    /// command-line. When merging, it is best practice to prioritize options
    /// from the cli to those saved on-disk. To do so, prefer keeping data
    /// fields from `self` when conflicting with `other`.
    fn merge(&mut self, other: Self);
}

/// Configuration object.
#[derive(Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, deny_unknown_fields)
)]
pub struct Config {
    /// Frontend options.
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub app: Frontend,

    /// Emulator options.
    #[cfg_attr(feature = "clap", clap(flatten))]
    pub emu: Emulator,
}

impl Join for Config {
    fn rebase(&mut self, root: &Path) {
        self.app.rebase(root);
        self.emu.rebase(root);
    }

    fn merge(&mut self, other: Self) {
        self.app.merge(other.app);
        self.emu.merge(other.emu);
    }
}

/// Deserializing configuration.
#[cfg(feature = "toml")]
pub mod de {
    use std::str::FromStr;

    pub use toml::de::Error;

    use crate::Config;

    impl FromStr for Config {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            toml::from_str(s)
        }
    }
}
