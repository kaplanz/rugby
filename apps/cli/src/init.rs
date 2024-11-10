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
use rugby::emu::part::video;
use rugby_cfg::opt;
#[cfg(feature = "gbd")]
use rugby_gbd::{Debugger, Filter};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, reload, EnvFilter, Layer, Registry};

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

impl App {
    /// Constructs a new `App`.
    pub fn new(args: &Cli) -> Result<Self> {
        // Initialize emulator
        let emu = self::emu(&args.cfg.data)?;

        // Initialize graphics
        let gui = args
            .feat
            .headless
            .not()
            .then(|| {
                self::gui(
                    #[cfg(feature = "win")]
                    args.dbg.win,
                )
            })
            .transpose()
            .context("could not open graphics")?;

        // Initialize link cable
        let lnk = args
            .feat
            .link
            .as_ref()
            .map(self::link)
            .transpose()
            .context("could not open link cable")?;

        // Initialize debugger
        #[cfg(feature = "gbd")]
        let gbd = args
            .dbg
            .gbd
            .then(self::gbd)
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

        // Install signal handler
        let sig = {
            // Use channels for communication
            let (tx, rx) = mpsc::channel();
            // Register SIGINT
            ctrlc::set_handler(move || {
                trace!("interrupt occurred");
                // Crash if channel has closed
                if let Err(err) = tx.send(()) {
                    panic!("failed to send interrupt: {err}");
                }
            })
            .context("could not register signal handler")?;
            // Use receiver as signal handle
            rx
        };

        // Construct application
        Ok(App {
            bye: args.feat.exit.then_some(app::Exit::CommandLine),
            cfg: app::Options {
                spd: args.cfg.data.app.spd.clone().unwrap_or_default().freq(),
            },
            ctx: None,
            emu,
            gui: app::Frontend {
                cfg: app::gui::Options {
                    pal: args.cfg.data.app.pal.clone().unwrap_or_default().into(),
                },
                win: gui,
                lnk,
            },
            sig,
            #[cfg(feature = "debug")]
            dbg: app::Debug {
                #[cfg(feature = "gbd")]
                gbd,
                #[cfg(feature = "log")]
                trace,
                #[cfg(feature = "win")]
                win: args.dbg.win,
            },
        })
    }
}

/// Dummy filter trait.
#[cfg(not(feature = "gbd"))]
pub trait Filter {}

/// Installs the global logger.
///
/// # Returns
///
/// Returns an handle which can be used to reload the logging filter.
pub fn log(filter: &str) -> Result<impl Filter> {
    /// Reload handle implementation.
    mod imp {
        #[allow(clippy::wildcard_imports)]
        use super::*;

        /// Tracing reload handle.
        type Reload = reload::Handle<EnvFilter, Registry>;

        /// Internal reload handle.
        #[cfg_attr(not(feature = "gbd"), allow(unused))]
        #[derive(Debug)]
        pub struct Handle {
            handle: Reload,
            filter: String,
        }

        impl Handle {
            pub fn new(reload: Reload) -> Self {
                Self {
                    filter: reload.with_current(ToString::to_string).unwrap(),
                    handle: reload,
                }
            }
        }

        impl Filter for Handle {
            #[cfg(feature = "gbd")]
            fn get(&self) -> &str {
                &self.filter
            }

            #[cfg(feature = "gbd")]
            fn set(&mut self, filter: String) {
                self.handle.reload(&filter).unwrap();
                self.filter = filter;
            }
        }
    }

    // Install logger
    let reload;
    tracing_subscriber::registry()
        .with({
            let (filter, handle) = reload::Layer::new(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::WARN.into())
                    .parse(filter)
                    .with_context(|| format!("failed to parse: {filter:?}"))?,
            );
            reload = handle;
            fmt::layer().with_filter(filter)
        })
        .try_init()?;

    // Wrap reload handle
    Ok(imp::Handle::new(reload))
}

/// Builds an emulator instance.
pub fn emu(cfg: &Config) -> Result<GameBoy> {
    // Load cart ROM
    let mut cart = self::cart(&cfg.emu.cart).context("invalid cartridge")?;
    // Load cart RAM
    if let Some(cart) = cart.as_mut() {
        util::rom::flash(&cfg.emu.cart, cart).context("invalid save RAM")?;
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
    if args.skip && args.rom.is_none() {
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
        debug!("read {nbytes} bytes: `{}`", path.display());
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
    Ok(Some(cart))
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
pub fn gbd() -> Result<Debugger> {
    // Construct a new `Debugger`
    let mut gbd = Debugger::new();
    // Initialize prompt handle
    gbd.prompt(Console::new().context("failed to initialize readline")?);
    // Enable by default
    gbd.enable();
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
