use std::collections::VecDeque;

use super::meta::{Meta, Pixel, Row};

/// Pixel FIFO.
#[derive(Clone, Debug, Default)]
pub struct Fifo(VecDeque<Pixel>);

#[expect(unused)]
impl Fifo {
    /// Constructs a new `Fifo`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears the FIFO, removing all pixels.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Pushes a row of pixels into the FIFO.
    #[expect(clippy::needless_pass_by_value)]
    pub fn push(&mut self, row: Row, meta: Meta) {
        let iter = std::mem::take(&mut self.0)
            .into_iter()
            .map(Some)
            .chain(std::iter::repeat(None))
            .zip(row)
            .map(|(old, new)| {
                let new = Pixel::new(new, meta.clone());
                if let Some(old) = old {
                    Pixel::blend(new, old)
                } else {
                    new
                }
            });
        self.0 = iter.collect();
    }

    /// Removes the next pixel from the FIFO and returns it.
    pub fn pop(&mut self) -> Option<Pixel> {
        self.0.pop_front()
    }

    /// Returns the number of pixels in the FIFO.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the FIFO is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the FIFO is full.
    pub fn is_full(&self) -> bool {
        self.0.len() >= 8
    }
}
