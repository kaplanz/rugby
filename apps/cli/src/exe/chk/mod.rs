//! Analyze provided ROM.

use anyhow::{Context, anyhow};
use constcat::concat;
use log::trace;
use rugby::core::dmg::cart;

use crate::app::{init, save};
use crate::err::Result;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-check");

/// [`Check`](crate::cli::Command::Chk) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init(None).context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Extract cartridge path
    let Some(path) = &args.cart.rom else {
        return Err(anyhow!("missing path to ROM image").into());
    };

    // Check cartridge header
    let head = {
        // Read ROM data
        let data =
            init::util::load_exact::<0x150>(path).context("unable to load cartridge header")?;
        // Load cartridge header
        cart::Header::new(&data).context("failed to construct cartridge header")?
    };
    println!(
        "{}",
        match args.fmt.unwrap_or_default() {
            cli::Format::Pretty => head.to_string(),
            cli::Format::Json =>
                serde_json::to_string_pretty(&head).context("unable to render cartridge header")?,
        }
    );

    // Skip cartridge body
    if args.head {
        return Ok(());
    }

    // Check cart ROM
    let mut cart = init::cart(&args.cart)?.context("try again with a valid ROM")?;
    // Check cart RAM
    save::ram::load(&args.cart, &mut cart).context("error flashing save RAM")?;

    Ok(())
}
