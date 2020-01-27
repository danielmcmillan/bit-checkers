mod checkers;
use checkers::board::{Board, Player1, Player2};

fn main() {
    let board = Board::new();

    board.legal_moves(Player1);
    // println!("bs1: {:?}", board);
}
