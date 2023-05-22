use crate::types::Color;

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    All,
}

impl Piece {
    pub fn from(c: char) -> Piece {
        let color = if c.is_lowercase() {
            Color::Black
        } else {
            Color::White
        };
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

    pub fn piece_type(&self) -> PieceType {
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
