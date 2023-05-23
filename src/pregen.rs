use std::ops::{Index, IndexMut};

use crate::types::*;
use crate::magic::*;

type IndexableBitboardList = [Bitboard; 64];

pub struct Pregen {
    pub attacks: PseudoAttacks,
    pub bishop_magics: MagicBitboard,
    pub rook_magics: MagicBitboard,
}

pub struct PseudoAttacks {
    pub knight: IndexableBitboardList,
    pub bishop: IndexableBitboardList,
    pub rook: IndexableBitboardList,
    pub queen: IndexableBitboardList,
    pub king: IndexableBitboardList,
}

impl Pregen {
    pub fn init() -> Pregen {
        let bishop = generate_magics(&MagicPiece::Bishop);
        let rook = generate_magics(&MagicPiece::Rook);
        Pregen {
            attacks: PseudoAttacks::init(&bishop, &rook),
            bishop_magics: bishop,
            rook_magics: rook,
        }
    }
}

impl PseudoAttacks {
    const fn new() -> PseudoAttacks {
        PseudoAttacks {
            knight: [Bitboard(0); 64],
            bishop: [Bitboard(0); 64],
            rook: [Bitboard(0); 64],
            queen: [Bitboard(0); 64],
            king: [Bitboard(0); 64],
        }
    }

    pub fn init(bishop_magics:& MagicBitboard, rook_magics: &MagicBitboard) -> PseudoAttacks {
        let mut attacks = PseudoAttacks::new();
        for sq in Square::all() {
            // calculate knight attacks
            for step in [-17, -15, -10, -6, 6, 10, 15, 17] {
                if !sq.safe_step(step) {
                    continue;
                };
                attacks.knight[sq + step] |= Bitboard::square(sq);
            }
            // calculate king attacks
            for step in [-9, -8, -7, -1, 1, 7, 8, 9] {
                if !sq.safe_step(step) {
                    continue;
                };
                attacks.king[sq + step] |= Bitboard::square(sq);
            }

            attacks.bishop[sq] = bishop_magics.get(sq)[Bitboard(0)];
            attacks.rook[sq] = rook_magics.get(sq)[Bitboard(0)];
            attacks.queen[sq] = attacks.bishop[sq] | attacks.rook[sq];
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
            PieceType::Knight => &self.knight,
            PieceType::Bishop => &self.bishop,
            PieceType::Rook => &self.rook,
            PieceType::Queen => &self.queen,
            PieceType::King => &self.king,
            _ => panic!("Invalid piece type"),
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
            PieceType::Knight => &mut self.knight,
            PieceType::Bishop => &mut self.bishop,
            PieceType::Rook => &mut self.rook,
            PieceType::Queen => &mut self.queen,
            PieceType::King => &mut self.king,
            _ => panic!("Invalid piece type"),
        }
    }
}

impl IndexMut<Piece> for PseudoAttacks {
    fn index_mut(&mut self, index: Piece) -> &mut [Bitboard; 64] {
        &mut self[index.piece_type()]
    }
}
