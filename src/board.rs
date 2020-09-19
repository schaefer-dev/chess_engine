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


#[cfg(test)]
mod tests {
    use crate::board;
    #[test]
    fn valid_field_creation_01() {
        let field1 = board::Field::new('a', 1);
        match field1 {
            Some(field) => {
                assert_eq!(field.file, 'A');
                assert_eq!(field.rank, 1);
            },
            None => panic!("Field was not created correctly")
        }
    }
    fn valid_field_creation_02() {
        let field1 = board::Field::new('h', 8);
        match field1 {
            Some(field) => {
                assert_eq!(field.file, 'H');
                assert_eq!(field.rank, 8);
            },
            None => panic!("Field was not created correctly")
        }
    }

    fn invalid_field_creation_01() {
        let field1 = board::Field::new('i', 1);
        match field1 {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
    }

    fn invalid_field_creation_02() {
        let field1 = board::Field::new('d', 9);
        match field1 {
            Some(field) => {
                panic!("Field was created despite being invalid")
            },
            None => ()
        }
    }
}