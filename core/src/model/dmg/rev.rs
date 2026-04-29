//! DMG-CPU silicon revision markers.
//!
//! These types identify revisions of the DMG-CPU chip itself. Mainboard
//! differences (audio circuitry, power supply filtering, etc.) vary
//! independently across production runs and are not modelled here.

use std::fmt::Display;

use crate::rev::sealed::Sealed;

/// DMG-CPU 0.
///
/// A rare early production variant, manufactured only through early 1989.[^1]
///
/// It is distinguished from later revisions primarily by its boot ROM.
/// Significant portions of the boot ROM code are rearranged relative to later
/// revisions. Both the logo and header checksum checks are performed before
/// displaying anything, and the ® symbol is absent from the Nintendo logo. On a
/// failed check, the screen blinks between solid white and black while the boot
/// ROM locks up. The post-boot register state also differs from later
/// revisions.[^2]
///
/// [^1]: <https://gbhwdb.gekkio.fi/consoles/dmg/>
/// [^2]: <https://gbdev.io/pandocs/Power_Up_Sequence.html>
#[derive(Debug, Default)]
pub struct Zero;

impl Display for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DMG-CPU 0")
    }
}

impl Sealed for Zero {}

/// DMG-CPU A.
///
/// The first mass-produced revision, manufactured from mid-1989.[^1]
///
/// Revisions A, B, and C share the same boot ROM and have no known behavioural
/// differences observable by software.[^2]
///
/// [^1]: <https://gbhwdb.gekkio.fi/consoles/dmg/>
/// [^2]: <https://gbdev.io/pandocs/Power_Up_Sequence.html>
#[derive(Debug, Default)]
pub struct A;

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DMG-CPU A")
    }
}

impl Sealed for A {}

/// DMG-CPU B.
///
/// A silicon revision of [DMG-CPU A](A), manufactured from early 1990.[^1]
///
/// Revisions A, B, and C share the same boot ROM and have no known behavioural
/// differences observable by software.[^2]
///
/// [^1]: <https://gbhwdb.gekkio.fi/consoles/dmg/>
/// [^2]: <https://gbdev.io/pandocs/Power_Up_Sequence.html>
pub type B = A;

/// DMG-CPU C.
///
/// A silicon revision of [DMG-CPU B](B), manufactured from late 1995.[^1]
///
/// Revisions A, B, and C share the same boot ROM and have no known behavioural
/// differences observable by software.[^2]
///
/// [^1]: <https://gbhwdb.gekkio.fi/consoles/dmg/>
/// [^2]: <https://gbdev.io/pandocs/Power_Up_Sequence.html>
pub type C = A;
