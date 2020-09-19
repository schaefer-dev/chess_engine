#[derive(Debug)]
#[derive(PartialEq)]
enum Figure {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}
#[derive(Debug)]
#[derive(PartialEq)]
enum Color {
    Black,
    White
}

#[derive(Debug)]
pub struct Piece {
    color: Color,
    figure: Figure,
}

impl Piece {
    pub fn new(white: bool, figure_char: char) -> Option<Piece> {
        let figure_char = figure_char.to_ascii_uppercase();


        let figure = match figure_char {
            'K' => Some(Figure::King),
            'Q' => Some(Figure::Queen),
            'B' => Some(Figure::Bishop),
            'N' => Some(Figure::Knight),
            'R' => Some(Figure::Rook),
            'P' => Some(Figure::Pawn),
            _ => None,
        };
        if figure == None {
            // Case of invalid figure character
            return None
        }

        let color = match white {
            true => Color::White,
            false => Color::Black,
        };

        Some(Piece {
            color: color,
            figure: figure.unwrap(),
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::piece;

    #[test]
    fn valid_piece_creation_series() {
        let piece1 = piece::Piece::new(true, 'k');
        match piece1 {
            Some(piece) => {
                assert_eq!(piece.color, piece::Color::White);
                assert_eq!(piece.figure, piece::Figure::King);
            },
            None => panic!("Piece was not created correctly")
        }

        let piece2 = piece::Piece::new(false, 'P');
        match piece2 {
            Some(piece) => {
                assert_eq!(piece.color, piece::Color::Black);
                assert_eq!(piece.figure, piece::Figure::Pawn);
            },
            None => panic!("Piece was not created correctly")
        }

        let piece3 = piece::Piece::new(false, 'q');
        match piece3 {
            Some(piece) => {
                assert_eq!(piece.color, piece::Color::Black);
                assert_eq!(piece.figure, piece::Figure::Queen);
            },
            None => panic!("Piece was not created correctly")
        }

        let piece4 = piece::Piece::new(false, 'R');
        match piece4 {
            Some(piece) => {
                assert_eq!(piece.color, piece::Color::Black);
                assert_eq!(piece.figure, piece::Figure::Rook);
            },
            None => panic!("Piece was not created correctly")
        }

        let piece5 = piece::Piece::new(true, 'n');
        match piece5 {
            Some(piece) => {
                assert_eq!(piece.color, piece::Color::White);
                assert_eq!(piece.figure, piece::Figure::Knight);
            },
            None => panic!("Piece was not created correctly")
        }
    }

    #[test]
    fn invalid_piece_creation_series() {
        let piece1 = piece::Piece::new(true, 'x');
        match piece1 {
            Some(piece) => {
                panic!("Piece was created despite being invalid")
            },
            None => ()
        }

        let piece2 = piece::Piece::new(true, '1');
        match piece2 {
            Some(piece) => {
                panic!("Piece was created despite being invalid")
            },
            None => ()
        }
    }
}