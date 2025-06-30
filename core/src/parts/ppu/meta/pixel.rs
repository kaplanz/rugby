use super::{Color, Palette};

/// Pre-rendered pixel.
#[derive(Clone, Debug)]
pub struct Pixel {
    /// Color value.
    pub col: Color,
    /// Pixel metadata.
    pub meta: Meta,
}

impl Pixel {
    /// Constructs a new `Pixel`.
    #[must_use]
    pub fn new(col: Color, meta: Meta) -> Self {
        Self { col, meta }
    }

    // Blends two pixels together.
    #[must_use]
    pub fn blend(a: Self, b: Self) -> Self {
        match (&a.meta, &b.meta) {
            (Meta::Bgw, Meta::Bgw) => select::color(a, b),
            (Meta::Bgw, Meta::Obj { .. }) => select::blend(a, b),
            (Meta::Obj { .. }, Meta::Bgw) => select::blend(b, a),
            (Meta::Obj { .. }, Meta::Obj { .. }) => select::overlap(a, b),
        }
    }
}

mod select {
    use super::{Color, Meta, Pixel};

    /// Select the first opaque pixel.
    #[must_use]
    pub fn color(a: Pixel, b: Pixel) -> Pixel {
        if a.col == Color::C0 { b } else { a }
    }

    /// Blend background and sprite pixels.
    #[expect(clippy::if_same_then_else)]
    #[must_use]
    pub fn blend(bgw: Pixel, obj: Pixel) -> Pixel {
        // Pixels are blended as follows:
        //
        // 1. If the color number of the sprite pixel is 0, the background pixel
        //    is used.
        if obj.col == Color::C0 {
            bgw
        }
        // 2. If the BG-to-OBJ priority bit is 1 and the color number of the
        //    background pixel is anything other than 0, the background pixel is
        //    used.
        else if bgw.col != Color::C0
            && let Meta::Obj { prty: true, .. } = obj.meta
        {
            bgw
        }
        // 3. If none of the above conditions apply, the sprite pixel is used.
        else {
            obj
        }
        // <https://hacktix.github.io/GBEDG/ppu/#pixel-mixing>
    }

    // Overlap two object pixels.
    #[must_use]
    pub fn overlap(a: Pixel, b: Pixel) -> Pixel {
        // If either is transparent, use the other
        if a.col == Color::C0 || b.col == Color::C0 {
            return color(a, b);
        }
        // Otherwise, use drawing priority such that:
        //
        // 1. Lower X-coordinates win
        match (&a.meta, &b.meta) {
            (Meta::Obj { xpos: xa, .. }, Meta::Obj { xpos: xb, .. }) => {
                if xa < xb {
                    a
                } else {
                    b
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Pixel metadata.
#[derive(Clone, Debug)]
pub enum Meta {
    /// Background/window metadata.
    Bgw,
    /// Object metadata.
    Obj {
        /// Object palette.
        objp: Palette,
        /// Background priority.
        prty: bool,
        /// X-coordinate.
        xpos: u8,
    },
}

impl Meta {
    /// Retrieve the palette.
    #[must_use]
    pub fn pal(&self) -> Palette {
        match self {
            Meta::Bgw => Palette::BgWin,
            Meta::Obj { objp, .. } => *objp,
        }
    }
}
