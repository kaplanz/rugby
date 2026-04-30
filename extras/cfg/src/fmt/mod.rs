//! Serialization formats.
//!
//! Each submodule handles one format and is gated behind the matching feature
//! flag:
//!
//! - [`toml`]: parse from a TOML string via [`std::str::FromStr`].

#[cfg(feature = "toml")]
pub mod toml;
