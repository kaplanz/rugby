//! Analyze provided ROM.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Context;
use constcat::concat;
use log::{debug, trace};
use rugby::core::dmg::cart;

use crate::app::{init, save};
use crate::err::Result;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-check");

/// [`Check`](crate::cli::Command::Check) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init(None).context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Check cartridge header
    let head = {
        // Read ROM data
        let path = args
            .cart
            .rom
            .as_ref()
            .context("missing path to ROM image")?;
        let data = self::load::<0x150>(path).context("unable to load cartridge header")?;
        // Load cartridge header
        cart::header::Header::new(&data).context("failed to construct valid header")?
    };
    println!(
        "{}",
        match args.fmt.unwrap_or_default() {
            cli::Format::Pretty => head.to_string(),
            cli::Format::Json =>
                serde_json::to_string_pretty(&head).context("unable to render cartridge header")?,
        }
    );

    // Load cart ROM
    let mut cart = init::cart(&args.cart)?.context("try again with a valid ROM")?;
    // Load cart RAM
    save::ram::load(&args.cart, &mut cart).context("error flashing save RAM")?;

    Ok(())
}

/// Loads a ROM from disk.
fn load<const N: usize>(path: &Path) -> Result<[u8; N]> {
    // Open ROM file
    let mut file =
        File::open(path).with_context(|| format!("failed to open: `{}`", path.display()))?;
    // Read ROM into a buffer
    let mut buf = [0u8; N];
    file.read_exact(&mut buf)
        .with_context(|| format!("failed to read: `{}`", path.display()))?;
    let nbytes = buf.len();
    debug!(
        "read {size}: `{path}`",
        size = bfmt::Size::from(nbytes),
        path = path.display(),
    );
    // Use ROM contents
    Ok(buf)
}
