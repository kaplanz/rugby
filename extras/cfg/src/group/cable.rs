//! Link cable configuration.

use std::net::SocketAddr;

use merge::Merge;

/// Cable options.
#[derive(Debug, Default, Merge)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(feature = "clap", command(next_help_heading = "Cable"))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, deny_unknown_fields)
)]
#[cfg_attr(
    all(feature = "facet", feature = "serde"),
    expect(clippy::unsafe_derive_deserialize)
)]
pub struct Cable {
    /// Link cable host address.
    ///
    /// Binds a local UDP socket to the specified address for serial
    /// communications.
    #[cfg_attr(
        feature = "clap",
        arg(long, value_name = "ADDR", required = false, requires = "peer")
    )]
    #[cfg_attr(feature = "serde", serde(skip))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub host: Option<SocketAddr>,

    /// Link cable peer address.
    ///
    /// Opens a UDP socket at the specified address for serial communications.
    #[cfg_attr(
        feature = "clap",
        arg(long, value_name = "ADDR", required = false, requires = "host")
    )]
    #[cfg_attr(feature = "serde", serde(skip))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub peer: Option<SocketAddr>,
}
