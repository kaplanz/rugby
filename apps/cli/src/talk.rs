//! Inter-thread communication.

use std::sync::mpsc;

use thiserror::Error;

/// Bi-directional communication channel.
#[derive(Debug)]
pub struct Channel<R, S> {
    /// Receiver channel.
    rx: mpsc::Receiver<R>,
    /// Sender channel.
    tx: mpsc::Sender<S>,
}

impl<R, S> Channel<R, S> {
    /// Send data through the channel.
    pub fn send(&mut self, data: S) -> Result<()> {
        self.tx.send(data).map_err(|_| Error::Send)
    }

    /// Receive data from the channel.
    pub fn recv(&mut self) -> Result<Option<R>> {
        match self.rx.try_recv() {
            Ok(data) => Ok(Some(data)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Awaits data from the channel (blocking).
    pub fn wait(&mut self) -> Result<R> {
        self.rx.recv().map_err(Into::into)
    }
}

/// Constructs a pair of communication channels.
pub fn pair<A, B>() -> (Channel<A, B>, Channel<B, A>) {
    let (txa, rxa) = mpsc::channel();
    let (txb, rxb) = mpsc::channel();
    (Channel { tx: txa, rx: rxb }, Channel { tx: txb, rx: rxa })
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by inter-thread communication.
#[derive(Debug, Error)]
pub enum Error {
    /// [Send](Channel::send) failed.
    #[error("receiver channel disconnected")]
    Send,
    /// [Recv](Channel::recv) failed.
    #[error("sender channel disconnected")]
    Recv(#[from] mpsc::TryRecvError),
    /// [Wait](Channel::wait) failed.
    #[error("sender channel disconnected")]
    Wait(#[from] mpsc::RecvError),
}
