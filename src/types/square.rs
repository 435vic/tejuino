use crate::types::{File, Rank};

const FILES: &[&'static str] = &["A", "B", "C", "D", "E", "F", "G", "H"];
const RANKS: &[&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8"];

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


impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= self.to {
            let square = ALL_SQUARES[self.index];
            self.index += self.step;
            Some(square)
        } else {
            None
        }
    }
}

impl Square {
    pub fn in_range(n: isize) -> bool {
        n >= 0 && n < 64
    }

    pub fn file(&self) -> File {
        File::try_from((*self as usize) & 7).expect("Square should be valid (0 <= square <= 63)")
    }

    pub fn rank(&self) -> Rank {
        Rank::try_from((*self as usize) >> 3).expect("Square should be valid (0 <= square <= 63)")
    }

    // Manhattan distance (king moves)
    pub fn dist(&self, to: Square) -> usize {
        let file_dist = (self.file() as isize).abs_diff(to.file() as isize);
        let rank_dist = (self.rank() as isize).abs_diff(to.rank() as isize);
        file_dist.max(rank_dist)
    }

    pub fn from(n: isize) -> Square {
        if n >= 0 && n < 64 {
            ALL_SQUARES[n as usize]
        } else {
            Square::InvalidSquare
        }
    }

    pub fn safe_step(&self, step: isize) -> bool {
        if Square::in_range(*self as isize + step) {
            let to = *self + step;
            self.dist(to) <= 2
        } else {
            false
        }
    }

    pub fn step(&self, step: isize) -> Option<Square> {
        if self.safe_step(step) {
            Some(*self + step)
        } else {
            None
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

impl From<Square> for usize {
    fn from(sq: Square) -> usize {
        sq as usize
    }
}

impl From<Square> for isize {
    fn from(sq: Square) -> isize {
        sq as isize
    }
}

impl std::ops::Add<isize> for Square {
    type Output = Square;

    fn add(self, rhs: isize) -> Self::Output {
        let n: isize = self.into();
        Square::from(n + rhs)
    }
}

impl std::ops::Sub<isize> for Square {
    type Output = Square;

    fn sub(self, rhs: isize) -> Self::Output {
        let n: isize = self.into();
        Square::from(n - rhs)
    }
}

impl std::ops::AddAssign<isize> for Square {
    fn add_assign(&mut self, rhs: isize) {
        *self = Square::from(*self as isize + rhs);
    }
}

impl std::ops::SubAssign<isize> for Square {
    fn sub_assign(&mut self, rhs: isize) {
        *self = Square::from(*self as isize - rhs);
    }
}


impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", FILES[self.file() as usize], RANKS[self.rank() as usize])
    }
}
