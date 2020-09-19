mod board;
mod piece;

fn main() {
    println!("Hello, world!");

    let mut board = board::Board::new();
    board.initial_position();

    println!("Board: {:?}", board);
}
