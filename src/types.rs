use crate::util;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Square {
    A1 =  0, B1, C1, D1, E1, F1, G1, H1,
    A2 =  8, B2, C2, D2, E2, F2, G2, H2,
    A3 = 16, B3, C3, D3, E3, F3, G3, H3,
    A4 = 24, B4, C4, D4, E4, F4, G4, H4,
    A5 = 32, B5, C5, D5, E5, F5, G5, H5,
    A6 = 40, B6, C6, D6, E6, F6, G6, H6,
    A7 = 48, B7, C7, D7, E7, F7, G7, H7,
    A8 = 56, B8, C8, D8, E8, F8, G8, H8,
    InvalidSquare = 64,
}

pub struct SquareIter {
    index: usize,
    to: usize,
    step: usize,
}

// All of the squares on the board
// Used for converting numbers to squares
const ALL_SQUARES: [Square; 64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

pub enum Direction {
    Up = 8,
    Right = 1,
    Down = -8,
    Left = -1,

    UpRight = 8 + 1,
    UpLeft = 8 - 1,
    DownRight = -8 + 1,
    DownLeft = -8 - 1,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

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

impl File {
    fn try_from(file: usize) -> Result<File, String> {
        match file {
            0 => Ok(File::FileA),
            1 => Ok(File::FileB),
            2 => Ok(File::FileC),
            3 => Ok(File::FileD),
            4 => Ok(File::FileE),
            5 => Ok(File::FileF),
            6 => Ok(File::FileG),
            7 => Ok(File::FileH),
            _ => Err(format!("Invalid file: {}", file)),
        }
    }
}

impl Rank {
    fn try_from(rank: usize) -> Result<Rank, String> {
        match rank {
            0 => Ok(Rank::Rank1),
            1 => Ok(Rank::Rank2),
            2 => Ok(Rank::Rank3),
            3 => Ok(Rank::Rank4),
            4 => Ok(Rank::Rank5),
            5 => Ok(Rank::Rank6),
            6 => Ok(Rank::Rank7),
            7 => Ok(Rank::Rank8),
            _ => Err(format!("Invalid rank: {}", rank)),
        }
    }
}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.to {
            let square = ALL_SQUARES[self.index];
            self.index += self.step;
            Some(square)
        } else {
            None
        }
    }
}

impl Square {
    pub fn in_range(n: usize) -> bool {
        n < 64
    }

    pub fn file(&self) -> File {
        File::try_from((*self as usize) & 7).expect("Square should be valid (0 <= square <= 63)")
    }

    pub fn rank(&self) -> Rank {
        Rank::try_from((*self as usize) >> 3).expect("Square should be valid (0 <= square <= 63)")
    }

    // Manhattan distance (king moves)
    pub fn dist(&self, to: &Square) -> usize {
        let file_dist = (self.file() as isize).abs_diff(to.file() as isize);
        let rank_dist = (self.rank() as isize).abs_diff(to.rank() as isize);
        file_dist.max(rank_dist)
    }

    pub fn from(n: usize) -> Square {
        if n < 64 {
            ALL_SQUARES[n as usize]
        } else {
            Square::InvalidSquare
        }
    }

    pub fn safe_step(self, step: isize) -> bool {
        let to_n = (self as isize) + step;
        if Square::in_range(to_n as usize) {
            let to = Square::from(to_n as usize);
            self.dist(&to) <= 2
        } else {
            false
        }
    }

    pub fn all() -> SquareIter {
        SquareIter {
            index: Square::A1 as usize,
            to: Square::H8 as usize,
            step: 1,
        }
    }

    pub fn iter(start: Square, end: Square, step: usize) -> SquareIter {
        SquareIter {
            index: start as usize,
            to: end as usize,
            step,
        }
    }
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

impl Bitboard {
    pub fn square(square: Square) -> Bitboard {
        Bitboard(1 << square as usize)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

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
        write!(f, "{}", util::render_grid(&chars, true))
    }
}
