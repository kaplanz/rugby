use std::ops::{Index, IndexMut};

use super::Color;

/// 8x8 pixel tile.
#[derive(Clone, Debug)]
pub struct Tile([Row; 8]);

impl Tile {
    /// Horizontally flip a tile.
    pub fn xflip(&mut self) {
        self.0.iter_mut().for_each(Row::xflip);
    }

    /// Vertically flip a tile.
    pub fn yflip(&mut self) {
        self.0.reverse();
    }
}

impl From<[Row; 8]> for Tile {
    fn from(rows: [Row; 8]) -> Self {
        Self(rows)
    }
}

impl From<[u8; 16]> for Tile {
    fn from(bytes: [u8; 16]) -> Self {
        Self(
            <[_; 8]>::try_from(bytes.as_chunks().0)
                .unwrap()
                .map(Row::from),
        )
    }
}

impl Index<usize> for Tile {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Tile {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl IntoIterator for Tile {
    type Item = Row;

    type IntoIter = <[Row; 8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// 8-pixel row.
///
/// This represents a single horizontal row of a [`Tile`].
#[derive(Clone, Debug)]
pub struct Row([Color; 8]);

impl Row {
    /// Horizontally flip a row.
    pub fn xflip(&mut self) {
        self.0.reverse();
    }

    /// Shift a row leftwards, filling with transparent pixels.
    #[rustfmt::skip]
    pub fn shift(&mut self, skip: usize) {
        self.0 = std::array::from_fn(|idx| {
            self.0.get(idx + skip).copied().unwrap_or(Color::C0)
        });
    }
}

impl From<[u8; 2]> for Row {
    fn from(data: [u8; 2]) -> Self {
        Self(
            std::array::from_fn(|idx| {
                // Extract color bits
                let mask = 1 << (7 - idx);
                let bit0 = data[0] & mask != 0;
                let bit1 = data[1] & mask != 0;
                // Combine into color value
                (u8::from(bit1) << 1) | u8::from(bit0)
            })
            .map(Color::from),
        )
    }
}

impl Index<usize> for Row {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl IntoIterator for Row {
    type Item = Color;
    type IntoIter = <[Color; 8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
