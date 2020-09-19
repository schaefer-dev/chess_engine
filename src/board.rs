use crate::piece;
use crate::field::Field;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    board_state: std::collections::HashMap<Field, piece::Piece>,
    move_history: Vec<(Field, Field)>,
    en_passant_possible: Vec<(Field)>,
    black_queenside_castling_possible: bool,
    black_kingside_castling_possible: bool,
    white_queenside_castling_possible: bool,
    white_kingside_castling_possible: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board_state: HashMap::new(),
            move_history: vec!(),
            en_passant_possible: vec!(),
            black_queenside_castling_possible: true,
            black_kingside_castling_possible: true,
            white_queenside_castling_possible: true,
            white_kingside_castling_possible: true,
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

    /// returns the set of possible fields which may be moved to. Empty if not no moves possible
    /// or if no figure is on the given field.
    pub fn get_possible_moves(&self, position: Field) -> Vec<Field> {
        let piece_opt = self.board_state.get(&position);

        match piece_opt {
            None => vec!(),
            Some(piece) => {
                match piece.figure {
                    piece::Figure::Pawn => {
                        let mut twoStepPossible = true;
                        match piece.color {
                            piece::Color::White => if position.rank != 2 {twoStepPossible = false},
                            piece::Color::Black => if position.rank != 7 {twoStepPossible = false},
                        }
                        let mut possible_fields: Vec<Field> = vec!();
                        // TODO: compute possible fields

                        possible_fields
                    }
                    piece::Figure::Rook => {
                        // TODO: implement
                        vec!()
                    }
                    piece::Figure::Knight => {
                        // TODO: implement
                        vec!()
                    }
                    piece::Figure::Bishop => {
                        // TODO: implement
                        vec!()
                    }
                    piece::Figure::Queen => {
                        // TODO: implement
                        vec!()
                    }
                    piece::Figure::King => {
                        // TODO: implement
                        vec!()
                    }
                }
            }
        }
    }

    pub fn get_figure_at_position(&self, file: char, rank: u8) -> Option<&piece::Piece> {
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
            match self.get_figure_at_position(*file, rank) {
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
