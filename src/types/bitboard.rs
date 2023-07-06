//! # Bitboards
//! 
//! Bitboards are a way to represent chess information in a single, 64 integer.
//! Not only are they very memory efficient, but they also allow for very fast bitwise operations,
//! which are used in move generation and evaluation.

use crate::types::{Square, Direction};
use crate::util;

/// The bitboard type. It is a 64 bit unsigned integer.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

pub struct BitboardIter {
    bitboard: Bitboard,
}

impl Bitboard {
    pub const RANK_1: Bitboard = Bitboard(0xFF);
    pub const RANK_2: Bitboard = Bitboard(0xFF << (8 * 1));
    pub const RANK_3: Bitboard = Bitboard(0xFF << (8 * 2));
    pub const RANK_4: Bitboard = Bitboard(0xFF << (8 * 3));
    pub const RANK_5: Bitboard = Bitboard(0xFF << (8 * 4));
    pub const RANK_6: Bitboard = Bitboard(0xFF << (8 * 5));
    pub const RANK_7: Bitboard = Bitboard(0xFF << (8 * 6));
    pub const RANK_8: Bitboard = Bitboard(0xFF << (8 * 7));

    pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
    pub const FILE_B: Bitboard = Bitboard(0x0101010101010101 << 1);
    pub const FILE_C: Bitboard = Bitboard(0x0101010101010101 << 2);
    pub const FILE_D: Bitboard = Bitboard(0x0101010101010101 << 3);
    pub const FILE_E: Bitboard = Bitboard(0x0101010101010101 << 4);
    pub const FILE_F: Bitboard = Bitboard(0x0101010101010101 << 5);
    pub const FILE_G: Bitboard = Bitboard(0x0101010101010101 << 6);
    pub const FILE_H: Bitboard = Bitboard(0x0101010101010101 << 7);

    /// Create a bitboard with a single square set.
    /// 
    /// # Example
    /// ```
    /// use tejuino::types::{Bitboard, Square};
    /// 
    /// let bitboard = Bitboard::square(Square::B1);
    /// assert_eq!(bitboard, Bitboard(0x2));
    /// ```
    #[inline]
    pub fn square(square: Square) -> Bitboard {
        Bitboard(1 << square as usize)
    }

    /// Create a bitboard from a list of squares.
    /// 
    /// # Example
    /// ```
    /// use tejuino::types::{Bitboard, Square};
    /// 
    /// let bitboard = Bitboard::squares(&[Square::A1, Square::B1, Square::C1, Square::D1]);
    /// assert_eq!(bitboard, Bitboard(0xF));
    /// ```
    pub fn squares(squares: &[Square]) -> Bitboard {
        let mut bitboard = Bitboard(0);
        for square in squares {
            bitboard |= Bitboard::square(*square);
        }
        bitboard
    }

    /// Generate a bitboard with the edges where the square provided _isn't_ set.
    /// Used for magic bitboard generation. (See: src/magic.rs)
    #[inline]
    pub fn edges(s: Square) -> Bitboard {
        (Bitboard::FILE_A | Bitboard::FILE_H) & !Bitboard::file(s) | 
        (Bitboard::RANK_1 | Bitboard::RANK_8) & !Bitboard::rank(s)
    }

    /// Generate a bitboard with the file the square provided is on set.
    #[inline]
    pub fn file(square: Square) -> Bitboard {
        Bitboard::FILE_A << (square.file()) as usize
    }

    /// Generate a bitboard with the rank the square provided is on set.
    #[inline]
    pub fn rank(square: Square) -> Bitboard {
        Bitboard::RANK_1 << (square.rank() as usize) * 8
    }

    /// Checks if a bitboard is empty.
    #[inline]
    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Shift a bitboard by a specified amount and direction. Set bits that go off the board are lost.
    #[inline]
    pub fn shift(&self, d: Direction, amt: usize) -> Bitboard {
        match d {
            Direction::Up => *self << (8 * amt),
            Direction::Down => *self >> (8 * amt),
            _ => *self,
        }
    }
    
    pub fn iter(&self) -> BitboardIter {
        BitboardIter {
            bitboard: *self,
        }
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard.0 == 0 {
            return None;
        }
        let square = self.bitboard.0.trailing_zeros() as usize;
        self.bitboard.0 &= self.bitboard.0 - 1;
        Some(Square::from(square))
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

impl std::ops::Shl<usize> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl std::ops::Shr<usize> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::ShrAssign<usize> for Bitboard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
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
        write!(f, "\n{}", util::render_grid(&chars, true))
    }
}
