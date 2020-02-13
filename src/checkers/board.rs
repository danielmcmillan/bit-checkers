use super::util::BitGrid;
use std::fmt;

const BOARD_WIDTH: u32 = 8;
const BOARD_HEIGHT: u32 = 8;
const BOARD_MASK: u64 = 0x55AA55AA55AA55AA;

#[derive(Clone, Copy, fmt::Debug)]
pub enum Player {
    Player1,
    Player2,
}
pub use Player::{Player1, Player2};

pub struct Board {
    player1: PlayerBoard,
    player2: PlayerBoard,
}

#[derive(Clone, Copy, fmt::Debug)]
pub struct Position(pub u32, pub u32);

#[derive(Clone, Copy, fmt::Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    fn new(from: Position, offset: (i32, i32)) -> Move {
        Move {
            from: Position(from.0, from.1),
            to: Position(
                ((from.0 as i32) + offset.0) as u32,
                ((from.1 as i32) + offset.1) as u32,
            ),
        }
    }
}

struct PlayerBoard {
    pub all: BitGrid,
    pub kings: BitGrid,
}

impl Board {
    /// Creates a new board with pieces in the initial positions.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::Board::new();
    /// ```
    pub fn new() -> Board {
        Board {
            player1: PlayerBoard {
                all: BitGrid::new_from_mask(BOARD_MASK << 40 >> 40),
                kings: BitGrid::new(),
            },
            player2: PlayerBoard {
                all: BitGrid::new_from_mask(BOARD_MASK >> 40 << 40),
                kings: BitGrid::new(),
            },
        }
    }

    pub fn move_piece(mut self, player: Player, Move { from, to }: Move) -> Board {
        // Todo: create kings
        let player_board = match player {
            Player1 => &mut self.player1,
            Player2 => &mut self.player2,
        };
        player_board.all = player_board
            .all
            .set_at_cell(from.0, from.1, false)
            .set_at_cell(to.0, to.1, true);
        if player_board.kings.get_at_cell(from.0, from.1) {
            player_board.kings = player_board
                .kings
                .set_at_cell(from.0, from.1, false)
                .set_at_cell(to.0, to.1, true);
        }

        self
    }

    pub fn normal_moves(&self, player: Player) -> impl Iterator<Item = Move> {
        let downward_moving = self.downward_moving(player);
        let upward_moving = self.upward_moving(player);

        let get_moves = |moving, vertical: i32, horizontal: i32| {
            self.empty_squares()
                .shift(-vertical, -horizontal)
                .intersect(moving)
                .iter_set_cells()
                .map(move |(x, y)| Move::new(Position(x, y), (horizontal, vertical)))
        };

        get_moves(downward_moving, 1, -1)
            .chain(get_moves(downward_moving, 1, 1))
            .chain(get_moves(upward_moving, -1, -1))
            .chain(get_moves(upward_moving, -1, 1))
    }

    pub fn jump_moves(&self, player: Player) -> impl Iterator<Item = Move> {
        let downward_moving = self.downward_moving(player);
        let upward_moving = self.upward_moving(player);
        let opponents = match player {
            Player1 => self.player2.all,
            Player2 => self.player1.all,
        };

        let get_jumps = |moving, vertical: i32, horizontal: i32| {
            self.empty_squares()
                .shift(-vertical * 2, -horizontal * 2)
                .intersect(opponents.shift(-vertical, -horizontal))
                .intersect(moving)
                .iter_set_cells()
                .map(move |(x, y)| Move::new(Position(x, y), (horizontal * 2, vertical * 2)))
        };

        get_jumps(downward_moving, 1, -1)
            .chain(get_jumps(downward_moving, 1, 1))
            .chain(get_jumps(upward_moving, -1, -1))
            .chain(get_jumps(upward_moving, -1, 1))
    }

    fn empty_squares(&self) -> BitGrid {
        self.player1
            .all
            .union(self.player2.all)
            .negate()
            .intersect(BitGrid::new_from_mask(BOARD_MASK))
    }

    fn downward_moving(&self, player: Player) -> BitGrid {
        match player {
            Player1 => self.player1.all,
            Player2 => self.player2.kings,
        }
    }

    fn upward_moving(&self, player: Player) -> BitGrid {
        match player {
            Player1 => self.player1.kings,
            Player2 => self.player2.all,
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
