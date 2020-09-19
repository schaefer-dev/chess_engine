mod board;
mod piece;

fn main() {

    let mut board = board::Board::new();
    board.initial_position();

    board.display_state_commandline();
}
