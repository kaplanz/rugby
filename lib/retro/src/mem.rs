//! Memory types.

use super::unsigned;

/// Passed to `retro_get_memory_{data,size}`.
///
/// If the memory type doesn't apply to the implementation NULL/0 can be
/// returned.
pub const RETRO_MEMORY_MASK: unsigned = 0xff;

/// Regular save RAM. This RAM is usually found on a game cartridge, backed up
/// by a battery.
///
/// If save game data is too complex for a single memory buffer, the
/// [`SAVE_DIRECTORY`] (preferably) or [`SYSTEM_DIRECTORY`] environment callback
/// can be used.
pub const RETRO_MEMORY_SAVE_RAM: unsigned = 0;

/// Some games have a built-in clock to keep track of time.
///
/// This memory is usually just a couple of bytes to keep track of time.
pub const RETRO_MEMORY_RTC: unsigned = 1;

/// System ram lets a frontend peek into a game systems main RAM.
pub const RETRO_MEMORY_SYSTEM_RAM: unsigned = 2;

/// Video ram lets a frontend peek into a game systems video RAM (VRAM).
pub const RETRO_MEMORY_VIDEO_RAM: unsigned = 3;
