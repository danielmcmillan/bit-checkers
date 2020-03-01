use std::fmt;

// TODO: more refined types for row/col
// TODO: generalise with future const generics feature

// Grid size and underlying type
type DataType = u64;
const GRID_COLS: u32 = 8;
const GRID_ROWS: u32 = 8;
const GRID_SIZE: u32 = GRID_COLS * GRID_ROWS;

#[derive(Clone, Copy)]
pub struct BitGrid {
  data: DataType,
}

/// Type for a fixed size 2d grid of bool values.
///
/// Grid size is 8 by 8.
/// Values can be accessed by index or by x and y coordinate. Values are in row-major order:
/// index 0 <=> x = 0, y = 0
/// index 1 <=> x = 1, y = 0
/// index 8 <=> x = 0, y = 1
impl BitGrid {
  /// Returns BitGrid with all values false.
  pub fn new() -> BitGrid {
    BitGrid { data: 0 }
  }

  /// Returns a BitGrid initialised based on a bit mask.
  /// The least significant bit will correspond to index 0.
  pub fn new_from_mask(data: DataType) -> BitGrid {
    BitGrid { data }
  }

  /// Returns the data at a specified index.
  ///
  /// Panics or returns undefined result if the index is invalid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new_from_mask(0b110);
  ///
  /// assert_eq!(grid.get_at_index(0), false);
  /// assert_eq!(grid.get_at_index(1), true);
  /// assert_eq!(grid.get_at_index(2), true);
  /// ```
  pub fn get_at_index(self, index: u32) -> bool {
    self.data & BitGrid::index_mask(index) != 0
  }

  /// Returns the data at a specified x and y coordinate.
  ///
  /// Panics or returns undefined result if the x and y coordinate are invalid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new_from_mask(0b01100000110);
  ///
  /// assert_eq!(grid.get_at_cell(0, 0), false);
  /// assert_eq!(grid.get_at_cell(1, 0), true);
  /// assert_eq!(grid.get_at_cell(0, 1), true);
  /// ```
  pub fn get_at_cell(self, x: u32, y: u32) -> bool {
    self.get_at_index(BitGrid::index_of_cell(x, y))
  }

  /// Returns a BitGrid with data set at the specified index.
  ///
  /// Panics or returns undefined result if the index is invalid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new()
  ///   .set_at_index(5, true);
  ///
  /// assert_eq!(grid.get_at_index(5), true);
  /// ```
  pub fn set_at_index(self, index: u32, value: bool) -> BitGrid {
    BitGrid {
      data: if value {
        self.data | BitGrid::index_mask(index)
      } else {
        self.data & !BitGrid::index_mask(index)
      },
    }
  }

  /// Returns a BitGrid with data set at the specified x and y coordinate.
  ///
  /// Panics or returns undefined result if the x and y coordinate are invalid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new()
  ///   .set_at_cell(5, 0, true);
  ///
  /// assert_eq!(grid.get_at_cell(5, 0), true);
  /// ```
  pub fn set_at_cell(self, x: u32, y: u32, value: bool) -> BitGrid {
    self.set_at_index(BitGrid::index_of_cell(x, y), value)
  }

  /// Returns a BitGrid with all values shiften by a specified number of rows and columns.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new()
  ///   .set_at_cell(1, 1, true)
  ///   .shift(1, 2);
  ///
  /// assert_eq!(grid.get_at_cell(1, 1), false, "Position before shift is false");
  /// assert_eq!(grid.get_at_cell(3, 2), true, "Position after shift is true");
  /// ```
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

  /// Returns a BitGrid with values being the intersection with the specified BitGrid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid1 = bit_checkers::checkers::util::BitGrid::new_from_mask(0b11001);
  /// let grid2 = bit_checkers::checkers::util::BitGrid::new_from_mask(0b10101);
  /// let intersection = grid1.intersect(grid2);
  ///
  /// assert_eq!(intersection.get_at_index(0), true);
  /// assert_eq!(intersection.get_at_index(1), false);
  /// assert_eq!(intersection.get_at_index(2), false);
  /// assert_eq!(intersection.get_at_index(3), false);
  /// assert_eq!(intersection.get_at_index(4), true);
  /// ```
  pub fn intersect(self, other: BitGrid) -> BitGrid {
    BitGrid {
      data: self.data & other.data,
    }
  }

  /// Returns a BitGrid with values being the union with the specified BitGrid.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid1 = bit_checkers::checkers::util::BitGrid::new_from_mask(0b11001);
  /// let grid2 = bit_checkers::checkers::util::BitGrid::new_from_mask(0b10101);
  /// let intersection = grid1.union(grid2);
  ///
  /// assert_eq!(intersection.get_at_index(0), true);
  /// assert_eq!(intersection.get_at_index(1), false);
  /// assert_eq!(intersection.get_at_index(2), true);
  /// assert_eq!(intersection.get_at_index(3), true);
  /// assert_eq!(intersection.get_at_index(4), true);
  /// ```
  pub fn union(self, other: BitGrid) -> BitGrid {
    BitGrid {
      data: self.data | other.data,
    }
  }

  /// Returns a BitGrid with every value negated.
  ///
  /// # Examples
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new_from_mask(0b01);
  /// let negation = grid.negate();
  ///
  /// assert_eq!(negation.get_at_index(0), false);
  /// assert_eq!(negation.get_at_index(1), true);
  /// ```
  pub fn negate(self) -> BitGrid {
    BitGrid { data: !self.data }
  }

  /// Returns an iterator over the index for all true values.
  ///
  /// # Example
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new_from_mask(0b10100);
  ///
  /// assert_eq!(grid.iter_set_indexes().collect::<Vec<u32>>(), vec![2, 4]);
  /// ```
  pub fn iter_set_indexes(self) -> SetIndexIterator {
    SetIndexIterator(self.data)
  }

  /// Returns an iterator over the (x, y) coordinates for all true values.
  ///
  /// # Example
  ///
  /// ```
  /// let grid = bit_checkers::checkers::util::BitGrid::new_from_mask(0b100000100);
  ///
  /// assert_eq!(grid.iter_set_cells().collect::<Vec<(u32, u32)>>(), vec![(2, 0), (0, 1)]);
  /// ```
  pub fn iter_set_cells(self) -> SetCellIterator {
    self.iter_set_indexes().map(BitGrid::cell_at_index)
  }

  /// Returns whether there are no true values (all values false).
  ///
  /// # Example
  ///
  /// ```
  /// let empty_grid = bit_checkers::checkers::util::BitGrid::new();
  /// let non_empty_grid = empty_grid.set_at_index(0, true);
  ///
  /// assert_eq!(empty_grid.none(), true);
  /// assert_eq!(non_empty_grid.none(), false);
  /// ```
  pub fn none(&self) -> bool {
    self.data == 0
  }

  fn index_mask(index: u32) -> DataType {
    1 << index
  }

  fn index_of_cell(x: u32, y: u32) -> u32 {
    x + (y * GRID_COLS)
  }

  fn cell_at_index(index: u32) -> (u32, u32) {
    (index % GRID_ROWS, index / GRID_ROWS)
  }
}

impl fmt::Debug for BitGrid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let bit_vec: Vec<u32> = self
      .into_iter()
      .map(|bit| if bit { 1 } else { 0 })
      .collect();
    for chunk in bit_vec.chunks(GRID_COLS as usize) {
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
  index: u32,
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

pub struct SetIndexIterator(DataType);

impl Iterator for SetIndexIterator {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.0 == 0 {
      None
    } else {
      let result = self.0.trailing_zeros();
      self.0 &= self.0 - 1;

      Some(result)
    }
  }
}

pub type SetCellIterator = std::iter::Map<SetIndexIterator, fn(u32) -> (u32, u32)>;
