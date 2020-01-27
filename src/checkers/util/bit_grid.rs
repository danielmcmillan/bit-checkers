use std::fmt;

// row-major order, with least significant bit being top-left cell
// TODO: think about restricted types for row/col
type DataType = u64;

const GRID_COLS: usize = 8;
const GRID_ROWS: usize = 8;
const GRID_SIZE: usize = GRID_COLS * GRID_ROWS;

#[derive(Clone, Copy)]
pub struct BitGrid {
  data: DataType,
}

impl BitGrid {
  pub fn new() -> BitGrid {
    BitGrid { data: 0 }
  }

  pub fn new_from_mask(mask: DataType) -> BitGrid {
    BitGrid { data: mask }
  }

  pub fn get_at_index(self, index: usize) -> bool {
    self.data & BitGrid::index_mask(index) != 0
  }

  pub fn get_at_cell(self, x: usize, y: usize) -> bool {
    self.get_at_index(BitGrid::cell_at(x, y))
  }

  pub fn set_at_index(self, index: usize, value: bool) -> BitGrid {
    BitGrid {
      data: if value {
        self.data | BitGrid::index_mask(index)
      } else {
        self.data & !BitGrid::index_mask(index)
      },
    }
  }

  pub fn set_at_cell(self, x: usize, y: usize, value: bool) -> BitGrid {
    self.set_at_index(BitGrid::cell_at(x, y), value)
  }

  pub fn shift(self, rows: i32, cols: i32) -> BitGrid {
    let directed_amount: i32 = rows * (GRID_COLS as i32) + cols;
    let amount = directed_amount.abs() as u32;

    BitGrid {
      data: if directed_amount < 0 {
        self.data >> amount
      } else {
        self.data << amount
      },
    }
  }

  pub fn intersect(self, other: BitGrid) -> BitGrid {
    BitGrid {
      data: self.data & other.data,
    }
  }

  pub fn union(self, other: BitGrid) -> BitGrid {
    BitGrid {
      data: self.data | other.data,
    }
  }

  pub fn negate(self) -> BitGrid {
    BitGrid { data: !self.data }
  }

  pub fn empty(self) -> bool {
    self.data == 0
  }

  // get collection of all bits that are set

  fn index_mask(index: usize) -> DataType {
    1 << index
  }

  fn cell_at(x: usize, y: usize) -> usize {
    x + (y * GRID_COLS)
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

/*
 * Iterator
 */

pub struct BitGridIter {
  grid: BitGrid,
  index: usize,
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

impl Iterator for BitGridIter {
  type Item = bool;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < GRID_SIZE {
      let result = self.grid.get_at_index(self.index);
      self.index += 1;
      Some(result)
    } else {
      None
    }
  }
}
