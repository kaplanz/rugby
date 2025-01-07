//! Localization.

use super::{int, unsigned};
// documentation uses
#[allow(unused_imports)]
use crate::*;

/* Returned from `retro_get_region`. */
pub const RETRO_REGION_NTSC: unsigned = 0;
pub const RETRO_REGION_PAL: unsigned = 1;

/// Identifiers for supported languages.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_LANGUAGE`]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i32)]
#[rustfmt::skip]
pub enum retro_language {
   RETRO_LANGUAGE_ENGLISH             = 0,
   RETRO_LANGUAGE_JAPANESE            = 1,
   RETRO_LANGUAGE_FRENCH              = 2,
   RETRO_LANGUAGE_SPANISH             = 3,
   RETRO_LANGUAGE_GERMAN              = 4,
   RETRO_LANGUAGE_ITALIAN             = 5,
   RETRO_LANGUAGE_DUTCH               = 6,
   RETRO_LANGUAGE_PORTUGUESE_BRAZIL   = 7,
   RETRO_LANGUAGE_PORTUGUESE_PORTUGAL = 8,
   RETRO_LANGUAGE_RUSSIAN             = 9,
   RETRO_LANGUAGE_KOREAN              = 10,
   RETRO_LANGUAGE_CHINESE_TRADITIONAL = 11,
   RETRO_LANGUAGE_CHINESE_SIMPLIFIED  = 12,
   RETRO_LANGUAGE_ESPERANTO           = 13,
   RETRO_LANGUAGE_POLISH              = 14,
   RETRO_LANGUAGE_VIETNAMESE          = 15,
   RETRO_LANGUAGE_ARABIC              = 16,
   RETRO_LANGUAGE_GREEK               = 17,
   RETRO_LANGUAGE_TURKISH             = 18,
   RETRO_LANGUAGE_SLOVAK              = 19,
   RETRO_LANGUAGE_PERSIAN             = 20,
   RETRO_LANGUAGE_HEBREW              = 21,
   RETRO_LANGUAGE_ASTURIAN            = 22,
   RETRO_LANGUAGE_FINNISH             = 23,
   RETRO_LANGUAGE_INDONESIAN          = 24,
   RETRO_LANGUAGE_SWEDISH             = 25,
   RETRO_LANGUAGE_UKRAINIAN           = 26,
   RETRO_LANGUAGE_CZECH               = 27,
   RETRO_LANGUAGE_CATALAN_VALENCIA    = 28,
   RETRO_LANGUAGE_CATALAN             = 29,
   RETRO_LANGUAGE_BRITISH_ENGLISH     = 30,
   RETRO_LANGUAGE_HUNGARIAN           = 31,
   RETRO_LANGUAGE_BELARUSIAN          = 32,
   RETRO_LANGUAGE_GALICIAN            = 33,
   RETRO_LANGUAGE_NORWEGIAN           = 34,
   RETRO_LANGUAGE_LAST,

   /// Defined to ensure that `sizeof(retro_language) == sizeof(int)`. Do not
   /// use.
   RETRO_LANGUAGE_DUMMY          = int::MAX,
}
