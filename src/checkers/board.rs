use super::util::BitGrid;
use std::fmt;

const BOARD_WIDTH: u32 = 8;
const BOARD_HEIGHT: u32 = 8;
const BOARD_MASK: u64 = 0x55AA55AA55AA55AA;

#[derive(Clone, Copy, PartialEq, fmt::Debug)]
pub enum Player {
    Player1,
    Player2,
}
pub use Player::{Player1, Player2};

/// Type representing a checkers board
#[derive(Clone)]
pub struct Board {
    player1: PlayerBoard,
    player2: PlayerBoard,
}

#[derive(Clone, Copy, PartialEq, fmt::Debug)]
pub struct Position(pub u32, pub u32);

#[derive(Clone, Copy, PartialEq, fmt::Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

#[derive(PartialEq)]
pub struct Piece {
    pub position: Position,
    pub player: Player,
    pub king: bool,
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

#[derive(Clone)]
struct PlayerBoard {
    pub all: BitGrid,
    pub kings: BitGrid,
}

impl Board {
    /// Returns a new board with pieces in the initial positions.
    ///
    /// Player1's pieces are in the first three rows, Player2's pieces are in the last 3 rows.
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

    /// Returns a new board containing the given pieces.
    ///
    /// Panics or returns an undefined result if the positions are invalid
    /// (e.g. positions outside bounds of board, multiple pieces with same position)
    pub fn new_with_pieces<T>(pieces: T) -> Board
    where
        T: IntoIterator<Item = Piece>,
    {
        let mut board = Board {
            player1: PlayerBoard {
                all: BitGrid::new(),
                kings: BitGrid::new(),
            },
            player2: PlayerBoard {
                all: BitGrid::new(),
                kings: BitGrid::new(),
            },
        };
        for piece in pieces {
            let player_board = board.player_board_mut(piece.player);
            let Position(x, y) = piece.position;
            assert!(
                !player_board.all.get_at_cell(x, y),
                "should not be multiple pieces in same position"
            );
            player_board.all = player_board.all.set_at_cell(x, y, true);
            if piece.king {
                player_board.kings = player_board.kings.set_at_cell(x, y, true);
            }
        }

        board
    }

    /// Returns a new board with a move applied to a particular player's piece.
    ///
    /// Panics or returns a board in an invalid state if the specified move is invalid.
    /// Pieces will be promoted to kings when appropriate.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::board::Board::new();
    /// let a_move = board.normal_moves(bit_checkers::board::Player1).next().unwrap();
    /// let board = board.move_piece(bit_checkers::board::Player1, a_move);
    /// ```
    pub fn move_piece(mut self, player: Player, Move { from, to }: Move) -> Board {
        // Todo: create kings
        let player_board = self.player_board_mut(player);
        // Move the flag in the players 'all' board
        player_board.all = player_board
            .all
            .set_at_cell(from.0, from.1, false)
            .set_at_cell(to.0, to.1, true);
        // Move the king flag if the moved piece is a king
        if player_board.kings.get_at_cell(from.0, from.1)
            || (player == Player1 && to.1 == BOARD_HEIGHT - 1)
            || (player == Player2 && to.1 == 0)
        {
            player_board.kings = player_board
                .kings
                .set_at_cell(from.0, from.1, false)
                .set_at_cell(to.0, to.1, true);
        }

        self
    }

    /// Returns an iterator over non-jump moves that the given player can make.
    /// Note: these are only allowed in a normal game if there are no jump moves available.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::board::Board::new();
    /// let a_move = board
    ///     .normal_moves(bit_checkers::board::Player1)
    ///     .next()
    ///     .unwrap();
    /// let bit_checkers::board::Move { from, to } = a_move;
    /// ```
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

    /// Returns an iterator over jump moves that the given player can make.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::board::Board::new();
    /// let jump_move = board
    ///     .jump_moves(bit_checkers::board::Player1)
    ///     .next();
    ///
    /// assert!(jump_move.is_none());
    /// ```
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

    /// Returns the winning player based on the current board, or None if the game is still in
    /// progress.
    ///
    /// # Examples
    ///
    /// ```
    /// let new_board = bit_checkers::board::Board::new();
    /// let complete_board = bit_checkers::board::Board::new_with_pieces(vec![
    ///     bit_checkers::board::Piece {
    ///         player: bit_checkers::board::Player1,
    ///         king: false,
    ///         position: bit_checkers::board::Position(5, 6),
    ///     }
    /// ]);
    ///
    /// assert!(new_board.winner().is_none());
    /// assert_eq!(complete_board.winner(), Some(bit_checkers::board::Player1));
    /// ```
    pub fn winner(&self) -> Option<Player> {
        if self.player1.all.none() {
            Some(Player2)
        } else if self.player2.all.none() {
            Some(Player1)
        } else {
            None
        }
    }

    /// Returns an iterator over all pieces on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::board::Board::new_with_pieces(vec![
    ///     bit_checkers::board::Piece {
    ///         player: bit_checkers::board::Player1,
    ///         king: false,
    ///         position: bit_checkers::board::Position(5, 6),
    ///     }
    /// ]);
    /// let piece = board.pieces_iter().next().unwrap();
    ///
    /// assert_eq!(piece.position, bit_checkers::board::Position(5, 6));
    pub fn pieces_iter(self) -> impl Iterator<Item = Piece> {
        [Player1, Player2].iter().flat_map(move |&player| {
            let player_board = self.player_board(player);
            let kings = player_board.kings;
            player_board.all.iter_set_cells().map(move |(x, y)| {
                let king = kings.get_at_cell(x, y);
                Piece {
                    position: Position(x, y),
                    player,
                    king,
                }
            })
        })
    }

    /// Returns the piece at the specified position.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = bit_checkers::board::Board::new();
    /// let piece = board.piece_at(bit_checkers::board::Position(1, 0));
    /// let empty_spot = board.piece_at(bit_checkers::board::Position(1, 4));
    ///
    /// assert_eq!(piece.unwrap().player, bit_checkers::board::Player1);
    /// assert!(empty_spot.is_none());
    pub fn piece_at(&self, position: Position) -> Option<Piece> {
        let Position(x, y) = position;
        if self.player1.kings.get_at_cell(x, y) {
            Some(Piece {
                position,
                player: Player1,
                king: true,
            })
        } else if self.player1.all.get_at_cell(x, y) {
            Some(Piece {
                position,
                player: Player1,
                king: false,
            })
        } else if self.player2.kings.get_at_cell(x, y) {
            Some(Piece {
                position,
                player: Player2,
                king: true,
            })
        } else if self.player2.all.get_at_cell(x, y) {
            Some(Piece {
                position,
                player: Player2,
                king: false,
            })
        } else {
            None
        }
    }

    /// Returns a BitGrid with flag set for valid, empty squares.
    fn empty_squares(&self) -> BitGrid {
        self.player1
            .all
            .union(self.player2.all)
            .negate()
            .intersect(BitGrid::new_from_mask(BOARD_MASK))
    }

    /// Returns a BitGrid with flag set for all of the given player's pieces that can move
    /// downwards (y+)
    fn downward_moving(&self, player: Player) -> BitGrid {
        match player {
            Player1 => self.player1.all,
            Player2 => self.player2.kings,
        }
    }

    /// Returns a BitGrid with flag set for all of the given player's pieces that can move
    /// upwards (y-)
    fn upward_moving(&self, player: Player) -> BitGrid {
        match player {
            Player1 => self.player1.kings,
            Player2 => self.player2.all,
        }
    }

    /// Returns a reference to the PlayerBoard struct for the given player.
    fn player_board(&self, player: Player) -> &PlayerBoard {
        match player {
            Player1 => &self.player1,
            Player2 => &self.player2,
        }
    }

    /// Returns a mutable reference to the PlayerBoard struct for the given player.
    fn player_board_mut(&mut self, player: Player) -> &mut PlayerBoard {
        match player {
            Player1 => &mut self.player1,
            Player2 => &mut self.player2,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_move_pieces() {
        let board = Board::new();
        let board2 = board.clone().move_piece(
            Player2,
            Move {
                from: Position(0, 5),
                to: Position(1, 4),
            },
        );
        let board3 = board2.clone().move_piece(
            Player1,
            Move {
                from: Position(5, 2),
                to: Position(4, 3),
            },
        );

        assert_eq!(board.piece_at(Position(0, 5)).unwrap().player, Player2);
        assert!(board2.piece_at(Position(0, 5)).is_none());
        assert!(board3.piece_at(Position(0, 5)).is_none());
        assert!(board.piece_at(Position(1, 4)).is_none());
        assert_eq!(board2.piece_at(Position(1, 4)).unwrap().player, Player2);
        assert_eq!(board2.piece_at(Position(1, 4)).unwrap().king, false);
        assert_eq!(board3.piece_at(Position(1, 4)).unwrap().player, Player2);

        assert_eq!(board.piece_at(Position(5, 2)).unwrap().player, Player1);
        assert_eq!(board2.piece_at(Position(5, 2)).unwrap().player, Player1);
        assert!(board3.piece_at(Position(5, 2)).is_none());
        assert!(board.piece_at(Position(4, 3)).is_none());
        assert!(board2.piece_at(Position(4, 3)).is_none());
        assert_eq!(board3.piece_at(Position(4, 3)).unwrap().player, Player1);
        assert_eq!(board3.piece_at(Position(4, 3)).unwrap().king, false);
    }

    #[test]
    fn should_king_pieces() {
        let board = Board::new_with_pieces(vec![
            Piece {
                player: Player1,
                king: false,
                position: Position(5, 6),
            },
            Piece {
                player: Player2,
                king: false,
                position: Position(2, 1),
            },
        ]);
        let board2 = board
            .move_piece(
                Player1,
                Move {
                    from: Position(5, 6),
                    to: Position(6, 7),
                },
            )
            .move_piece(
                Player2,
                Move {
                    from: Position(2, 1),
                    to: Position(1, 0),
                },
            );
        let pieces: Vec<Piece> = board2.pieces_iter().collect();
        assert_eq!(pieces.len(), 2);
        assert!(pieces.contains(&Piece {
            position: Position(6, 7),
            player: Player1,
            king: true
        }));
        assert!(pieces.contains(&Piece {
            position: Position(1, 0),
            player: Player2,
            king: true
        }));
    }

    #[test]
    fn should_get_normal_moves() {
        let board = Board::new_with_pieces(vec![
            Piece {
                player: Player1,
                king: true,
                position: Position(3, 0),
            },
            Piece {
                player: Player1,
                king: false,
                position: Position(0, 3),
            },
            Piece {
                player: Player1,
                king: false,
                position: Position(7, 4),
            },
            Piece {
                player: Player1,
                king: false,
                position: Position(4, 5),
            },
            Piece {
                player: Player1,
                king: true,
                position: Position(3, 6),
            },
            Piece {
                player: Player1,
                king: true,
                position: Position(4, 7),
            },
            Piece {
                player: Player2,
                king: true,
                position: Position(5, 0),
            },
            Piece {
                player: Player2,
                king: true,
                position: Position(3, 4),
            },
            Piece {
                player: Player2,
                king: false,
                position: Position(2, 5),
            },
        ]);

        let player1_moves: Vec<Move> = board.normal_moves(Player1).collect();
        let player2_moves: Vec<Move> = board.normal_moves(Player2).collect();

        assert_eq!(player1_moves.len(), 7);
        assert!(player1_moves.contains(&Move::new(Position(3, 0), (-1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(3, 0), (1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(0, 3), (1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(7, 4), (-1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(4, 5), (1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(3, 6), (-1, 1))));
        assert!(player1_moves.contains(&Move::new(Position(4, 7), (1, -1))));

        assert_eq!(player2_moves.len(), 5);
        assert!(player2_moves.contains(&Move::new(Position(5, 0), (1, 1))));
        assert!(player2_moves.contains(&Move::new(Position(5, 0), (-1, 1))));
        assert!(player2_moves.contains(&Move::new(Position(3, 4), (1, -1))));
        assert!(player2_moves.contains(&Move::new(Position(3, 4), (-1, -1))));
        assert!(player2_moves.contains(&Move::new(Position(2, 5), (-1, -1))));
    }

    #[test]
    fn should_get_jump_moves() {
        let board = Board::new_with_pieces(vec![
            Piece {
                player: Player1,
                king: true,
                position: Position(0, 5),
            },
            Piece {
                player: Player1,
                king: false,
                position: Position(2, 5),
            },
            Piece {
                player: Player1,
                king: false,
                position: Position(3, 6),
            },
            Piece {
                player: Player1,
                king: true,
                position: Position(0, 7),
            },
            Piece {
                player: Player2,
                king: true,
                position: Position(1, 4),
            },
            Piece {
                player: Player2,
                king: false,
                position: Position(1, 6),
            },
            Piece {
                player: Player2,
                king: false,
                position: Position(4, 7),
            },
        ]);

        let player1_moves: Vec<Move> = board.jump_moves(Player1).collect();
        let player2_moves: Vec<Move> = board.jump_moves(Player2).collect();

        assert_eq!(player1_moves.len(), 2);
        assert!(player1_moves.contains(&Move::new(Position(0, 5), (2, -2))));
        assert!(player1_moves.contains(&Move::new(Position(0, 5), (2, 2))));

        assert_eq!(player2_moves.len(), 1);
        assert!(player2_moves.contains(&Move::new(Position(1, 6), (2, -2))));
    }
}
