use super::Palette;
use super::pixel::Meta;

/// Sprite metadata.
#[derive(Clone, Debug)]
pub struct Sprite {
    /// Byte 0: Y Position.
    pub ypos: u8,
    /// Byte 1: X Position.
    pub xpos: u8,
    /// Byte 2: Tile Index.
    pub tnum: u8,
    /// Byte 3: Attributes.
    pub attr: Attributes,
}

impl Sprite {
    /// Constructs a new `Sprite`.
    #[must_use]
    pub fn new(data: [u8; 4]) -> Self {
        Self::from(data)
    }

    /// Extracts sprite metadata.
    #[must_use]
    pub fn meta(&self) -> Meta {
        Meta::Obj {
            objp: Palette::objp(self.attr.objp),
            prty: self.attr.prty,
            xpos: self.xpos,
        }
    }
}

impl From<[u8; 4]> for Sprite {
    fn from(data: [u8; 4]) -> Self {
        Self {
            ypos: data[0],
            xpos: data[1],
            tnum: data[2],
            attr: data[3].into(),
        }
    }
}

/// Sprite attributes.
///
/// Attributes are encoded as `0bZYXP0000`, where:
/// - `P` is the object palette.
/// - `X` is the x-flip flag.
/// - `Y` is the y-flip flag.
/// - `Z` is the priority flag.
#[expect(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct Attributes {
    /// Priority flag.
    ///
    /// When set, background/window pixels have priority. (Other than
    /// [`C0`](super::Color::C0), which is always transparent.)
    pub prty: bool,
    /// Y-flip.
    ///
    /// Vertically flips the sprite.
    pub yflip: bool,
    /// X-flip.
    ///
    /// Horizontally flips the sprite.
    pub xflip: bool,
    /// Object palette.
    ///
    /// Selects between using `obp0` or `obp1`.
    pub objp: bool,
}

impl Attributes {
    /// Constructs a new `Attributes`.
    #[must_use]
    pub fn new(byte: u8) -> Self {
        Self::from(byte)
    }
}

impl From<u8> for Attributes {
    #[rustfmt::skip]
    fn from(byte: u8) -> Self {
        Self {
            prty:  byte & (1 << 7) != 0,
            yflip: byte & (1 << 6) != 0,
            xflip: byte & (1 << 5) != 0,
            objp:  byte & (1 << 4) != 0,
        }
    }
}
