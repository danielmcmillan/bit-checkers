use bit_checkers::{Board, Move, Player1, Player2, Position};

fn main() {
    let board = Board::new();
    let board = board.move_piece(
        Player1,
        Move {
            from: Position(1, 2),
            to: Position(1, 4),
        },
    );

    println!("{:?}", board);
    println!("Player1 normal moves:");
    for m in board.normal_moves(Player1) {
        println!("{:?}", m);
    }
    println!("Player2 normal moves:");
    for m in board.normal_moves(Player2) {
        println!("{:?}", m);
    }
    println!("Player1 jump moves:");
    for m in board.jump_moves(Player1) {
        println!("{:?}", m);
    }
    println!("Player2 jump moves:");
    for m in board.jump_moves(Player2) {
        println!("{:?}", m);
    }
}
