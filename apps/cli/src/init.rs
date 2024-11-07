//! Application initialization.

use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;
use std::ops::Not;
use std::path::Path;

use anyhow::{anyhow, ensure, Context, Result};
use log::{debug, info, warn};
use rugby::core::dmg::{Boot, Cartridge, GameBoy, LCD};
use rugby::emu::part::video;
use rugby_cfg::opt::emu::Cart;
#[cfg(feature = "gbd")]
use rugby_gbd::{Debugger, Portal};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::app::gui::Cable;
use crate::app::{self, App, Graphics};
use crate::cfg::Config;
#[cfg(feature = "gbd")]
use crate::dbg::gbd::Console;
#[cfg(feature = "log")]
use crate::dbg::log::Tracer;
#[cfg(feature = "log")]
use crate::exe::run::cli::trace::Trace;
use crate::exe::run::{cli, Cli};
use crate::{util, NAME};

/// Logging filter reload handle.
#[cfg(feature = "gbd")]
type Log = Portal<String>;
#[cfg(not(feature = "gbd"))]
type Log = ();

/// Installs the global logger.
///
/// # Returns
///
/// Returns an handle which can be used to reload the logging filter.
pub fn log(filter: &str) -> Result<Log> {
    // Construct logger
    let log = tracing_subscriber::fmt()
        .with_env_filter({
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .parse(filter)
                .with_context(|| format!("failed to parse: {filter:?}"))?
        })
        .with_filter_reloading();
    // Extract handle
    #[cfg(feature = "gbd")]
    let handle = {
        Portal {
            get: {
                let handle = log.reload_handle();
                Box::new(move || handle.with_current(ToString::to_string).unwrap())
            },
            set: {
                let handle = log.reload_handle();
                Box::new(move |filter: String| handle.reload(filter).unwrap())
            },
        }
    };
    #[cfg(not(feature = "gbd"))]
    let handle = ();
    // Install logger
    log.init();
    // Return handle
    Ok(handle)
}

/// Builds an emulator instance.
pub fn emu(cfg: &Config) -> Result<GameBoy> {
    // Load cartridge
    let mut cart = cfg
        .emu
        .cart
        .force
        .not()
        .then(|| self::cart(&cfg.emu.cart))
        .transpose()
        .context("could not load cartridge")?;
    // Flash cartridge RAM
    util::rom::flash(
        cfg.emu.cart.ram().as_deref(),
        cart.as_mut(),
        cfg.emu.cart.save.unwrap_or_default(),
    )
    .context("could not flash RAM")?;
    // Load boot ROM
    let boot = (|| {
        let boot = &cfg.emu.boot;
        boot.skip
            .not()
            .then(|| boot.rom.as_deref().map(self::boot))
            .map(|res| res.ok_or(anyhow!("missing path; unspecified by `--boot`")))
            .transpose()?
            .transpose()
    })() // NOTE: This weird syntax is in lieu of using unstable try blocks.
    .context("could not load boot ROM")?;

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
pub fn boot(path: &Path) -> Result<Boot> {
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
        debug!("read {nbytes} bytes: `{}`", path.display());
        // Use ROM contents
        buf
    };

    // Initialize boot ROM
    let boot = Boot::from(rom);
    info!("loaded boot ROM");

    // Return success
    Ok(boot)
}

/// Builds a cartridge instance.
pub fn cart(args: &Cart) -> Result<Cartridge> {
    // Extract path
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
        debug!("read {nbytes} bytes: `{}`", path.display());
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
    info!("loaded cartridge:\n{}", cart.header());

    // Return success
    Ok(cart)
}

/// Builds an application instance.
#[allow(unused_variables)]
pub fn app(args: &Cli, emu: GameBoy, log: Log) -> Result<App> {
    // Initialize graphics
    let gui = args
        .feat
        .headless
        .not()
        .then(|| {
            gui(
                #[cfg(feature = "win")]
                args.dbg.win,
            )
        })
        .transpose()
        .context("could not open graphics")?;

    // Open link cable
    let lnk = args
        .feat
        .link
        .as_ref()
        .map(link)
        .transpose()
        .context("could not open link cable")?;

    // Prepare debugger
    #[cfg(feature = "gbd")]
    let gbd = args
        .dbg
        .gbd
        .then(|| gbd(log))
        .transpose()
        .context("could not prepare debugger")?;

    // Initialize tracing
    #[cfg(feature = "log")]
    let trace = args
        .dbg
        .trace
        .as_ref()
        .map(trace)
        .transpose()
        .context("could not open trace logfile")?;

    // Construct application
    let app = App {
        cfg: app::Options {
            spd: args.cfg.data.app.spd.clone().unwrap_or_default().freq(),
        },
        #[cfg(feature = "debug")]
        dbg: app::Debug {
            #[cfg(feature = "gbd")]
            gbd,
            #[cfg(feature = "log")]
            trace,
            #[cfg(feature = "win")]
            win: args.dbg.win,
        },
        emu,
        gui: app::Frontend {
            cfg: app::gui::Options {
                pal: args.cfg.data.app.pal.clone().unwrap_or_default().into(),
            },
            win: gui,
            lnk,
        },
    };

    // Return app
    Ok(app)
}

/// Builds a graphics instance.
pub fn gui(#[cfg(feature = "win")] dbg: bool) -> Result<Graphics> {
    // Calculate aspect
    let video::Aspect { wd, ht } = LCD;
    let size = (wd.into(), ht.into()).into();
    // Construct GUI
    #[cfg_attr(not(feature = "win"), allow(unused_mut))]
    let mut gui = Graphics::new(NAME, size).context("failed to open main window")?;
    // Open debug windows
    #[cfg(feature = "win")]
    if dbg {
        gui.dbg.all().context("failed to open debug windows")?;
    };
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
        .context("failed to set non-blocking")?;
    // Return completed link
    Ok(sock)
}

/// Builds a debugger instance.
#[cfg(feature = "gbd")]
pub fn gbd(log: Log) -> Result<Debugger> {
    // Construct a new `Debugger`
    let mut gbd = Debugger::new();
    // Initialize prompt handle
    gbd.prompt(Box::new({
        Console::new().context("failed to initialize readline")?
    }));
    // Initialize logger handle
    gbd.logger(log);
    // Return constructed debugger
    Ok(gbd)
}

/// Builds a tracing instance.
#[cfg(feature = "log")]
pub fn trace(Trace { fmt, log }: &Trace) -> Result<Tracer> {
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
