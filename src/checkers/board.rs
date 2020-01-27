use crate::checkers::util::BitGrid;
use std::fmt;

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
const BOARD_MASK: u64 = 0x55AA55AA55AA55AA;

pub enum Player {
    Player1,
    Player2,
}
pub use Player::{Player1, Player2};

pub struct Board {
    player1: PlayerBoard,
    player2: PlayerBoard,
}

struct PlayerBoard {
    pub all: BitGrid,
    pub kings: BitGrid,
}

impl Board {
    // get all legal moves (must be jump if possible)
    // perform a move or jump (assuming it is legal)

    pub fn new() -> Board {
        Board {
            player1: PlayerBoard::new(false),
            player2: PlayerBoard::new(true),
        }
    }

    pub fn legal_moves(&self, player: Player) {
        let available_squares = self
            .player1
            .all
            .union(self.player2.all)
            .negate()
            .intersect(BitGrid::new_from_mask(BOARD_MASK));

        let downward_moving = match player {
            Player1 => self.player1.all,
            Player2 => self.player2.kings,
        };
        let upward_moving = match player {
            Player1 => self.player1.kings,
            Player2 => self.player2.all,
        };
        let opponents = match player {
            Player1 => self.player2.all,
            Player2 => self.player1.all,
        };

        // Down-left moves
        let result = available_squares.shift(-1, 1).intersect(downward_moving);
        println!("{:?}", result);
        // Down-right moves
        let result = available_squares.shift(-1, -1).intersect(downward_moving);
        println!("{:?}", result);
        // Up-left moves
        let result = available_squares.shift(1, 1).intersect(upward_moving);
        println!("{:?}", result);
        // Up-right moves
        let result = available_squares.shift(1, -1).intersect(upward_moving);
        println!("{:?}", result);

        // Down-left jumps
        let result = available_squares
            .shift(-2, 2)
            .intersect(opponents.shift(-1, 1))
            .intersect(downward_moving);
        println!("Down-left jumps: {:?}", result);
    }
}

impl PlayerBoard {
    fn new(player2: bool) -> PlayerBoard {
        PlayerBoard {
            all: BitGrid::new_from_mask(if player2 {
                BOARD_MASK >> 40 << 40
            } else {
                BOARD_MASK << 40 >> 40
            }),
            kings: BitGrid::new(),
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..BOARD_HEIGHT {
            writeln!(f)?;
            for x in 0..BOARD_WIDTH {
                let should_be_empty = (x % 2) == (y % 2);
                let man_1 = self.player1.all.get_at_cell(x, y);
                let king_1 = self.player1.kings.get_at_cell(x, y);
                let man_2 = self.player2.all.get_at_cell(x, y);
                let king_2 = self.player2.kings.get_at_cell(x, y);
                write!(
                    f,
                    "{}",
                    match (should_be_empty, man_1, king_1, man_2, king_2) {
                        (false, true, true, false, false) => "(1)",
                        (false, true, false, false, false) => " 1 ",
                        (false, false, false, true, true) => "(2)",
                        (false, false, false, true, false) => " 2 ",
                        (false, false, false, false, false) => " _ ",
                        (true, false, false, false, false) => "   ",
                        _ => panic!("Board in invalid state"),
                    }
                )?;
            }
        }
        Ok(())
    }
}
