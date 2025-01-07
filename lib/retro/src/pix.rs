//! Pixel format.

use super::int;
// documentation uses
#[allow(unused_imports)]
use crate::*;

/// The pixel format used for rendering.
///
/// # See
/// - [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i32)]
#[rustfmt::skip]
pub enum retro_pixel_format {
    /// `0RGB1555`, native endian.
    ///
    /// Used as the default if [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`] is not
    /// called.
    ///
    /// The most significant bit must be set to 0.
    ///
    /// # Deprecated
    ///
    /// This format remains supported to maintain compatibility. New code should
    /// use [`RETRO_PIXEL_FORMAT_RGB565`](Self::RETRO_PIXEL_FORMAT_RGB565)
    /// instead.
    ///
    /// # See
    ///
    /// - [`RETRO_PIXEL_FORMAT_RGB565`](Self::RETRO_PIXEL_FORMAT_RGB565)
    #[deprecated]
    RETRO_PIXEL_FORMAT_0RGB1555 = 0,

    /// `XRGB8888`, native endian.
    ///
    /// The most significant byte (the `X`) is ignored.
    RETRO_PIXEL_FORMAT_XRGB8888 = 1,

    /// `RGB565`, native endian.
    ///
    /// This format is recommended if 16-bit pixels are desired, as it is
    /// available on a variety of devices and APIs.
    RETRO_PIXEL_FORMAT_RGB565   = 2,

    /// Defined to ensure that `sizeof(retro_pixel_format) == sizeof(int)`. Do
    /// not use.
    #[deprecated]
    RETRO_PIXEL_FORMAT_UNKNOWN  = int::MAX,
}
