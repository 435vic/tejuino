pub mod pregen {
    use std::ops::{Index, IndexMut};

    use crate::types::*;

    type IndexableBitboardList = [Bitboard; 64];

    pub struct PseudoAttacks {
        pub none: IndexableBitboardList,
        pub pawn: IndexableBitboardList,
        pub knight: IndexableBitboardList,
        pub bishop: IndexableBitboardList,
        pub rook: IndexableBitboardList,
        pub queen: IndexableBitboardList,
        pub king: IndexableBitboardList,
        pub all: IndexableBitboardList,
    }

    impl PseudoAttacks {
        const fn new() -> PseudoAttacks {
            PseudoAttacks {
                none: [Bitboard(0); 64],
                pawn: [Bitboard(0); 64],
                knight: [Bitboard(0); 64],
                bishop: [Bitboard(0); 64],
                rook: [Bitboard(0); 64],
                queen: [Bitboard(0); 64],
                king: [Bitboard(0); 64],
                all: [Bitboard(0); 64],
            }
        }

        pub fn init() -> PseudoAttacks {
            let mut attacks = PseudoAttacks::new();
            for sq in Square::all() {
                // calculate knight attacks
                for step in [-17, -15, -10, -6, 6, 10, 15, 17] {
                    if !sq.safe_step(step) {
                        continue;
                    };
                    let to = (sq as isize + step) as usize;
                    attacks.knight[to] |= Bitboard::square(sq);
                }
                // calculate king attacks
                for step in [-9, -8, -7, -1, 1, 7, 8, 9] {
                    if !sq.safe_step(step) {
                        continue;
                    };
                    let to = (sq as isize + step) as usize;
                    attacks.king[to] |= Bitboard::square(sq);
                }
            }
            attacks
        }
    }

    impl Index<Square> for IndexableBitboardList {
        type Output = Bitboard;

        fn index(&self, index: Square) -> &Self::Output {
            &self[index as usize]
        }
    }

    impl IndexMut<Square> for IndexableBitboardList {
        fn index_mut(&mut self, index: Square) -> &mut Self::Output {
            &mut self[index as usize]
        }
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
                PieceType::All => &self.all,
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
                PieceType::All => &mut self.all,
            }
        }
    }

    impl IndexMut<Piece> for PseudoAttacks {
        fn index_mut(&mut self, index: Piece) -> &mut [Bitboard; 64] {
            &mut self[index.piece_type()]
        }
    }
}
