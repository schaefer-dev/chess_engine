mod board;
mod piece;
mod field;

fn main() {

    let mut board = board::Board::new();
    board.initial_position();

    board.display_state_commandline();

    // move pawn to b3
    let field_origin = field::Field::new('b', 2).unwrap();
    let field_dest = field::Field::new('b', 3).unwrap();
    board.move_piece(field_origin, field_dest);

    board.display_state_commandline();
}
