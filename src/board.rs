use crate::piece;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    board_state: HashMap<Field, Option<piece::Piece>>
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_state: HashMap::new()
        }
    }

    pub fn get_position(file: char, rank: u8) {

    }

    pub fn initial_position(&mut self) {

    }
}

#[derive(Debug)]
pub struct Field {
    file: char,
    rank: u8,
}

impl Field {
    pub fn new(file: char, rank: u8) -> Option<Field> {
        let file = file.to_ascii_uppercase();
        if (file > 'H' || file < 'A') {
            None
        } else {
            if (rank > 8) {
                None
            } else {
                Some(Field { file: file, rank: rank })
            }
        }
    }
}