mod board;
mod piece;
mod field;

fn main() {

    let mut board = board::Board::new();
    board.initial_position();

    board.display_state_commandline();
}
