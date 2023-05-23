use crate::types::Square;
use crate::util;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

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

    pub fn edges(s: Square) -> Bitboard {
        (Bitboard::FILE_A | Bitboard::FILE_H) & !Bitboard::file(s) | 
        (Bitboard::RANK_1 | Bitboard::RANK_8) & !Bitboard::rank(s)
    }

    pub fn file(square: Square) -> Bitboard {
        Bitboard::FILE_A << (square.file()) as usize
    }

    pub fn rank(square: Square) -> Bitboard {
        Bitboard::RANK_1 << (square.rank() as usize) * 8
    }

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn and(&self, other: &Bitboard) -> Bitboard {
        *self & *other
    }

    pub fn or(&self, other: &Bitboard) -> Bitboard {
        *self | *other
    }

    pub fn xor(&self, other: &Bitboard) -> Bitboard {
        *self ^ *other
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
        write!(f, "{}", util::render_grid(&chars, true))
    }
}
