pub mod data {
    use std::ops::{Index, IndexMut};

    use crate::{Bitboard, Piece, PieceType};
    pub struct PseudoAttacks {
        none: [Bitboard; 64],
        pawn: [Bitboard; 64],
        knight: [Bitboard; 64],
        bishop: [Bitboard; 64],
        rook: [Bitboard; 64],
        queen: [Bitboard; 64],
        king: [Bitboard; 64],
        all: [Bitboard; 64]
    }

    impl Index<PieceType> for PseudoAttacks {
        type Output = [Bitboard; 64];

        fn index(&self, index: PieceType) -> &Self::Output {
            match index {
                PieceType::None => &self.none,
                PieceType::Pawn => &self.pawn,
                PieceType::Knight => &self.knight,
                PieceType::Bishop => &self.bishop,
                PieceType::Rook => &self.rook,
                PieceType::Queen => &self.queen,
                PieceType::King => &self.king,
                PieceType::All => &self.all
            }
        }
    }

    impl Index<Piece> for PseudoAttacks {
        type Output = [Bitboard; 64];

        fn index(&self, index: Piece) -> &Self::Output {
            &self[index.piece_type()]
        }
    }

    impl IndexMut<PieceType> for PseudoAttacks {
        fn index_mut(&mut self, index: PieceType) -> &mut [Bitboard; 64] {
            match index {
                PieceType::None => &mut self.none,
                PieceType::Pawn => &mut self.pawn,
                PieceType::Knight => &mut self.knight,
                PieceType::Bishop => &mut self.bishop,
                PieceType::Rook => &mut self.rook,
                PieceType::Queen => &mut self.queen,
                PieceType::King => &mut self.king,
                PieceType::All => &mut self.all
            }
        }
    }

    impl IndexMut<Piece> for PseudoAttacks {
        fn index_mut(&mut self, index: Piece) -> &mut [Bitboard; 64] {
            &mut self[index.piece_type()]
        }
    }
}

