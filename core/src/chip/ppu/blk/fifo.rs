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
        row.into_iter().enumerate().for_each(|(idx, col)| {
            let new = Pixel::new(col, meta.clone());
            // Where does this pixel belong?
            if idx < self.0.len() {
                // Blend onto queued pixel
                self.0[idx] = Pixel::blend(new, self.0[idx].clone());
            } else {
                // Extend queue with pixel
                self.0.push_back(new);
            }
        });
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
