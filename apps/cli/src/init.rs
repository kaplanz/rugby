//! Application initialization.

use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;
use std::ops::Not;
#[cfg(feature = "log")]
use std::path::Path;
use std::sync::mpsc;

use anyhow::{ensure, Context, Result};
use log::{debug, info, trace, warn};
use rugby::core::dmg::{Boot, Cartridge, GameBoy, LCD};
use rugby::pal::Palette;
use rugby_cfg::opt;
#[cfg(feature = "gbd")]
use rugby_gbd::Debugger;

use crate::app::{self, App};
use crate::cfg::Config;
#[cfg(feature = "gbd")]
use crate::dbg::gbd::Console;
#[cfg(feature = "log")]
use crate::dbg::log::Tracer;
#[cfg(feature = "log")]
use crate::exe::run::cli::trace::Trace;
use crate::exe::run::{self, cli, Cli};
use crate::gui::{self, Cable};
use crate::util;

/// Builds an application instance
pub fn app(args: &Cli) -> Result<App> {
    // Initialize graphics
    let gui = args
        .feat
        .headless
        .not()
        .then_some(args)
        .map(self::gui)
        .transpose()
        .context("graphics initialization failed")?;

    // Initialize link cable
    let lnk = args
        .feat
        .link
        .as_ref()
        .map(self::link)
        .transpose()
        .context("link cable initialization failed")?;

    // Install signal handler
    let sig = {
        // Use channels for communication
        let (tx, rx) = mpsc::channel();
        // Register SIGINT
        ctrlc::set_handler(move || {
            trace!("interrupt signalled");
            // Crash if channel has closed
            tx.send(()).expect("receiver channel disconnected");
        })
        // Consider failed registration an application error
        .expect("error registering signal handler");
        // Use receiver as signal handle
        rx
    };

    // Construct application
    Ok(App {
        bye: args.feat.exit.then_some(app::Exit::CommandLine),
        gui: gui::Frontend {
            cfg: gui::Options {
                pal: args.cfg.data.app.pal.clone().unwrap_or_default().into(),
            },
            win: gui,
            lnk,
        },
        sig,
    })
}

/// Builds an emulator instance.
pub fn emu(cfg: &Config) -> Result<GameBoy> {
    // Load cart ROM
    let mut cart = self::cart(&cfg.emu.cart)
        .context("invalid cartridge")?
        .inspect(|cart| info!("cartridge header:\n{}", cart.header()));
    // Load cart RAM
    if let Some(cart) = cart.as_mut() {
        util::rom::flash(&cfg.emu.cart, cart).context("error flashing save RAM")?;
    }
    // Load boot ROM
    let boot = self::boot(&cfg.emu.boot).context("invalid boot ROM")?;

    // Instantiate emulator
    let mut emu = boot.map_or_else(GameBoy::new, GameBoy::with);
    // Insert cartridge
    if let Some(cart) = cart {
        emu.insert(cart);
    } else {
        // Handle missing cartridge
        ensure!(
            cfg.emu.cart.force,
            "missing cartridge; did not specify `--force`"
        );
        warn!("missing cartridge");
    }

    // Return emulator
    Ok(emu)
}

/// Builds a boot ROM instance.
pub fn boot(args: &opt::emu::Boot) -> Result<Option<Boot>> {
    // Allow none if skipped
    if args.skip || args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let path = args.rom.as_deref().context("missing path to ROM image")?;

    // Read ROM file
    let rom = {
        // Open ROM file
        let mut file =
            File::open(path).with_context(|| format!("failed to open: `{}`", path.display()))?;
        // Read ROM into a buffer (must be exactly 256 bytes)
        let mut buf = [0u8; 0x0100];
        file.read_exact(&mut buf)
            .with_context(|| format!("failed to read: `{}`", path.display()))?;
        let nbytes = buf.len();
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );
        // Use ROM contents
        buf
    };

    // Initialize boot ROM
    let boot = Boot::from(rom);
    info!("loaded boot ROM");

    // Return success
    Ok(Some(boot))
}

/// Builds a cartridge instance.
pub fn cart(args: &opt::emu::Cart) -> Result<Option<Cartridge>> {
    // Allow none if forced
    if args.force && args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let path = args.rom.as_deref().context("missing path to ROM image")?;

    // Read ROM file
    let rom = {
        // Open ROM file
        let file =
            File::open(path).with_context(|| format!("failed to open: `{}`", path.display()))?;
        // Read ROM into a buffer
        let mut buf = Vec::new();
        let nbytes = file
            // Game Paks manufactured by Nintendo have a maximum 8 MiB ROM
            .take(0x0080_0000)
            .read_to_end(&mut buf)
            .with_context(|| format!("failed to read: `{}`", path.display()))?;
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );
        // Use ROM contents
        buf
    };

    // Initialize cartridge
    let cart = if args.force {
        // If both force and check are supplied, default to force
        if args.check {
            warn!("use of `--force` overrides `--check`");
        }
        // Force cartridge construction
        Cartridge::unchecked
    } else if args.check {
        // Check cartridge integrity
        Cartridge::checked
    } else {
        // Construct a cartridge
        Cartridge::new
    }(&rom)
    .with_context(|| format!("failed to load: `{}`", path.display()))?;
    info!("loaded cartridge");

    // Return success
    Ok(Some(cart))
}

/// Builds a graphics instance.
pub fn gui(args: &run::Cli) -> Result<gui::Graphics> {
    // Construct GUI
    let mut gui = gui::Graphics::new().context("could not open main window")?;
    // Set initial title
    gui.lcd.title(util::title(&args.cfg.data.emu.cart));
    // Open debug windows
    #[cfg(feature = "win")]
    if args.dbg.win {
        gui.dbg.open().context("could not open debug windows")?;
    };
    // Draw initial window
    let pal = Palette::from(args.cfg.data.app.pal.clone().unwrap_or_default());
    gui.lcd.redraw(&vec![pal[0].into(); LCD.depth()])?;
    // Return GUI
    Ok(gui)
}

/// Builds a link cable instance.
pub fn link(cli::Link { host, peer }: &cli::Link) -> Result<Cable> {
    // Bind host to local address
    let sock =
        UdpSocket::bind(host).with_context(|| format!("failed to bind local socket: `{host}`"))?;
    // Connect to peer address
    sock.connect(peer)
        .with_context(|| format!("failed to connect to peer: `{peer}`"))?;
    // Set socket options
    sock.set_nonblocking(true)
        .context("could not set non-blocking")?;
    // Return completed link
    Ok(sock)
}

/// Builds a debugger instance.
#[cfg(feature = "gbd")]
pub fn gbd() -> Result<Debugger> {
    // Construct a new `Debugger`
    let mut gbd = Debugger::new();
    // Install logger reload handle
    gbd.logger(
        crate::log::RELOAD
            .get()
            .cloned()
            // unable to get is an application error
            .expect("unable to get logger handle"),
    );
    // Initialize prompt handle
    gbd.prompt(Console::new().context("error initializing readline")?);
    // Enable by default
    gbd.enable();
    // Return constructed debugger
    Ok(gbd)
}

/// Builds a tracing instance.
#[cfg(feature = "log")]
pub fn log(Trace { fmt, log }: &Trace) -> Result<Tracer> {
    let log = match log.as_deref() {
        // Create a logfile from the path
        Some(path) if path != Path::new("-") => either::Either::Left({
            File::create(path).with_context(|| format!("failed to open: `{}`", path.display()))?
        }),
        // Use `stdout` missing path or as alias of "-"
        _ => either::Either::Right(std::io::stdout()),
    };
    Ok(Tracer::new(*fmt, log))
}
