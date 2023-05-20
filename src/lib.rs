pub mod util;
pub mod board;
pub mod types;
mod movegen;

use types::*;

pub const BOARD_START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // turn bitboard into a vector of chars to use in util::render_grid
        // a circle or other char for 1, a space for 0
        let mut chars: [char; 64] = [' '; 64];
        for i in 0..64 {
            if self.0 & (1 << i) != 0 {
                chars[i] = '●';
            }
        }
        write!(f, "{}", util::render_grid(&chars))
    }
}

impl Piece {
    pub fn from(c: char) -> Piece {
        let color = if c.is_lowercase() { Color::Black } else { Color::White };
        match c.to_lowercase().to_string().as_str() {
            "p" => Piece::Pawn(color),
            "n" => Piece::Knight(color),
            "b" => Piece::Bishop(color),
            "r" => Piece::Rook(color),
            "q" => Piece::Queen(color),
            "k" => Piece::King(color),
            _ => Piece::None,
        }
    }

    pub fn id(&self) -> char {
        match self {
            Piece::Pawn(Color::White)   => 'P', Piece::Pawn(Color::Black)   => 'p',
            Piece::Knight(Color::White) => 'N', Piece::Knight(Color::Black) => 'n',
            Piece::Bishop(Color::White) => 'B', Piece::Bishop(Color::Black) => 'b',
            Piece::Rook(Color::White)   => 'R', Piece::Rook(Color::Black)   => 'r',
            Piece::Queen(Color::White)  => 'Q', Piece::Queen(Color::Black)  => 'q',
            Piece::King(Color::White)   => 'K', Piece::King(Color::Black)   => 'k',
            Piece::None => ' '
        }
    }

    fn piece_type(&self) -> PieceType {
        match self {
            Piece::None => PieceType::None,
            Piece::Pawn(_) => PieceType::Pawn,
            Piece::Knight(_) => PieceType::Knight,
            Piece::Bishop(_) => PieceType::Bishop,
            Piece::Rook(_) => PieceType::Rook,
            Piece::Queen(_) => PieceType::Queen,
            Piece::King(_) => PieceType::King,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_square() {
        assert_eq!(Square::E4.file(), File::FileE);
        assert_eq!(Square::E4.rank(), Rank::Rank4);

        // square methods
        assert_eq!(Square::E4.dist(&Square::E4), 0);
        assert_eq!(Square::E4.dist(&Square::E5), 1);
    }

    #[test]
    #[should_panic]
    fn invalid_square() {
    }
}
