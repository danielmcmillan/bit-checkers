use bit_checkers::{Board, Move, Player1, Player2, Position};

fn main() {
    let board = Board::new();

    board.legal_moves(Player1);
    // println!("bs1: {:?}", board);
}
