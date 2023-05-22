use crate::types::Square;
use crate::util;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn square(square: Square) -> Bitboard {
        Bitboard(1 << square as usize)
    }

    pub fn squares(squares: &[Square]) -> Bitboard {
        let mut bitboard = Bitboard(0);
        for square in squares {
            bitboard |= Bitboard::square(*square);
        }
        bitboard
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
