//! Application runtime.

use std::{process, thread};

use anyhow::Result;
use log::{debug, error, trace};

use crate::app::{self, Exit};
use crate::exe::run::Cli;

pub mod aux;
pub mod emu;
pub mod gui;
pub mod tui;

/// Run application.
///
/// # Note
///
/// In order to satisfy constraints of Cocoa-based systems, this function must
/// be called from the main thread.
#[expect(irrefutable_let_patterns)]
pub fn main(args: &Cli) -> Result<()> {
    // Replace default panic hook
    //
    // Ensure all threads exit when any panic by modifying the panic hook to
    // signal global application exit.
    if let panic = std::panic::take_hook() {
        std::panic::set_hook(Box::new(move |info| {
            // Exit all threads
            app::exit(Exit::Unknown);
            // Run default hook
            panic(info);
        }));
    }

    // Install signal handler
    ctrlc::try_set_handler(move || {
        trace!("application interrupted");
        // Attempt graceful exit (with cleanup) on first signal
        if app::running() {
            debug!("attempting graceful exit");
            app::exit(Exit::Interrupt);
        }
        // Abort application (without cleanup) if hanging
        else {
            error!("process terminated");
            process::abort();
        }
    })
    .expect("unable to register signal handler");

    // Process exit flag
    if args.feat.exit {
        // At this point calling `exit` doesn't do anything, as no threads
        // will be polling it yet.
        //
        // The purpose of the `--exit` flag is to return early after executing
        // all application startup. This can be achieved by prematurely marking
        // the program ready for exit, causing all threads to terminate as soon
        // as they complete initialization.
        app::exit(Exit::CliFlag);
    }

    // Spin up application threads
    thread::scope(|s| {
        // Run playback thread
        let aux = s.spawn(watch(|| aux::main(args)));
        // Run emulator thread
        let emu = s.spawn(watch(|| emu::main(args)));
        // Run terminal thread
        let tui = s.spawn(watch(|| tui::main(args)));
        // Run frontend thread
        //
        // Since on Cocoa-based systems, windows must be managed on the main
        // thread, the frontend entrypoint is run directly on this thread.
        let gui = watch(|| gui::main(args))();

        // Join threads after exit
        let aux = aux.join().expect("playback thread panicked");
        let emu = emu.join().expect("emulator thread panicked");
        let tui = tui.join().expect("terminal thread panicked");
        // Log exit reason
        debug!(
            "exit reason: {}",
            app::reason().expect("missing exit reason")
        );
        // Propagate errors
        aux.and(emu).and(tui).and(gui)
    })
}

/// Monitors a thread for errors.
pub fn watch<F>(main: F) -> impl FnOnce() -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    move || {
        // Execute entrypoint
        let res = main();
        // Handle thread errors
        if res.is_err() {
            // Trigger app exit
            app::exit(Exit::Error);
        }
        // Propagate result
        res
    }
}
