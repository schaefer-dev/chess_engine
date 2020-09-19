mod board;
mod piece;

fn main() {
    println!("Hello, world!");

    let board = board::Board::new();

    println!("Board: {:?}", board);

    let field_a1 = board::Field::new('a', 1);
    println!("Field: {:?}", field_a1);


    let field_invalid_1 = board::Field::new('a', 9);
    println!("Field: {:?}", field_invalid_1);
    let field_invalid_2 = board::Field::new('i', 3);
    println!("Field: {:?}", field_invalid_2);
}
