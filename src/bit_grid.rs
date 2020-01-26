use std::fmt;

// row-major order, with least significant bit being bottom-right cell
// TODO: think about restricted types for row/col
const GRID_COLS: usize = 4;
const GRID_SIZE: usize = 32;

#[derive(Clone, Copy)]
pub struct BitGrid {
  data: u32,
}

impl BitGrid {
  pub fn new() -> BitGrid {
    BitGrid { data: 0 }
  }

  pub fn get_index(&self, index: usize) -> bool {
    self.data & (1 << index) != 0
  }

  pub fn get(&self, x: usize, y: usize) -> bool {
    self.get_index(BitGrid::cell_at(x, y))
  }

  pub fn set(&mut self, x: usize, y: usize, value: bool) {
    let val = 1 << BitGrid::cell_at(x, y);
    if value {
      self.data |= val;
    } else {
      self.data &= !val;
    }
  }

  fn cell_at(x: usize, y: usize) -> usize {
    GRID_SIZE - 1 - x - (y * GRID_COLS)
  }
}

impl IntoIterator for BitGrid {
  type Item = bool;
  type IntoIter = BitGridIter;

  fn into_iter(self) -> Self::IntoIter {
    BitGridIter {
      grid: self,
      index: 0,
    }
  }
}

impl fmt::Debug for BitGrid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let bit_vec: Vec<u32> = self
      .into_iter()
      .map(|bit| if bit { 1 } else { 0 })
      .collect();
    for chunk in bit_vec.chunks(GRID_COLS) {
      write!(f, "\n{:?}", chunk)?;
    }
    Ok(())
  }
}

pub struct BitGridIter {
  grid: BitGrid,
  index: usize,
}

impl Iterator for BitGridIter {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < GRID_SIZE {
      self.index += 1;
      Some(self.grid.get_index(GRID_SIZE - self.index))
    } else {
      None
    }
  }
}
