#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black
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
    DownLeft = -8 - 1
}

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
    fn try_from(file: u8) -> Result<File, String> {
        match file {
            0 => Ok(File::FileA),
            1 => Ok(File::FileB),
            2 => Ok(File::FileC),
            3 => Ok(File::FileD),
            4 => Ok(File::FileE),
            5 => Ok(File::FileF),
            6 => Ok(File::FileG),
            7 => Ok(File::FileH),
            _ => Err(format!("Invalid file: {}", file))
        }
    }
}

impl Rank {
    fn try_from(rank: u8) -> Result<Rank, String> {
        match rank {
            0 => Ok(Rank::Rank1),
            1 => Ok(Rank::Rank2),
            2 => Ok(Rank::Rank3),
            3 => Ok(Rank::Rank4),
            4 => Ok(Rank::Rank5),
            5 => Ok(Rank::Rank6),
            6 => Ok(Rank::Rank7),
            7 => Ok(Rank::Rank8),
            _ => Err(format!("Invalid rank: {}", rank))
        }
    }
}

impl Square {
    pub fn in_range(&self) -> bool {
        *self >= Square::A1 && *self <= Square::H8
    }

    pub fn file(&self) -> File {
        File::try_from((*self as u8) & 7).expect("Square should be valid (0 <= square <= 63)")
    }

    pub fn rank(&self) -> Rank {
        Rank::try_from((*self as u8) >> 3).expect("Square should be valid (0 <= square <= 63)")
    }

    // Manhattan distance (king moves)
    pub fn dist(&self, to: &Square) -> u8 {
        let file_dist = (self.file() as i8).abs_diff(to.file() as i8);
        let rank_dist = (self.rank() as i8).abs_diff(to.rank() as i8);
        file_dist.max(rank_dist) as u8
    }

    pub fn from(n: u8) -> Square {
        if n < 64 {
            ALL_SQUARES[n as usize]
        } else {
            Square::InvalidSquare
        }
    }
}
