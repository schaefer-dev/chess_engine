use crate::piece;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    board_state: std::collections::HashMap<Field, piece::Piece>
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_state: HashMap::new()
        }
    }

    pub fn add_piece(&mut self, position: Field, piece: piece::Piece) {
        self.board_state.insert(position, piece);
    }

    pub fn clear_field(&mut self, position: Field) {
        self.board_state.remove_entry(&position);
    }

    fn reset(&mut self) {
        self.board_state.clear()
    }

    pub fn get_position(&self, file: char, rank: u8) -> Option<&piece::Piece> {
        let fieldo = Field::new(file.to_ascii_uppercase(), rank);
        if fieldo == None {
            None
        } else {
            let field = fieldo.unwrap();
            self.board_state.get(&field)
        }
    }

    pub fn display_state_commandline(&self) {
        print!("  ---------------------------------\n");
        for rank in &[8, 7, 6, 5, 4, 3, 2, 1] {
            self.print_rank(*rank);
            print!("\n  ---------------------------------\n");
        }
        print!("    A   B   C   D   E   F   G   H  \n");
    }

    fn print_rank(&self, rank: u8) {
        print!("{} |", rank);
        for file in &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            match self.get_position(*file, rank) {
                Some(piece) => print!(" {} |", piece.command_line_character()),
                None => print!("   |"),
            }
        }
    }

    pub fn initial_position(&mut self) {
        self.reset();
        let pieces_sequence = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];
        let mut file_iter: usize = 0;
        for file in &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            let field_w = Field::new(*file, 1);
            let field_b = Field::new(*file, 8);
            let field_wp = Field::new(*file, 2);
            let field_bp = Field::new(*file, 7);
            let piece_w = piece::Piece::new(true, pieces_sequence[file_iter]);
            let piece_b = piece::Piece::new(false, pieces_sequence[file_iter]);
            let piece_wp = piece::Piece::new(true, 'p');
            let piece_bp = piece::Piece::new(false, 'p');
            self.add_piece(field_w.unwrap(), piece_w.unwrap());
            self.add_piece(field_b.unwrap(), piece_b.unwrap());
            self.add_piece(field_wp.unwrap(), piece_wp.unwrap());
            self.add_piece(field_bp.unwrap(), piece_bp.unwrap());
            file_iter += 1
        }
    }
}

#[derive(Debug)]
#[derive(Hash)]
pub struct Field {
    file: char,
    rank: u8,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        (self.file == other.file) && (self.rank == other.rank)
    }
}
impl Eq for Field {}

impl Field {
    pub fn new(file: char, rank: u8) -> Option<Field> {
        let file = file.to_ascii_uppercase();
        if file > 'H' || file < 'A' {
            None
        } else {
            if rank > 8 {
                None
            } else {
                Some(Field { file, rank })
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