//! # Magic Bitboard Types
//! Types for magic bitboards.

use std::ops::Index;

use crate::types::{Bitboard, PieceType, Square};
use crate::magic::magic_index;

pub static RNG_SEEDS: &[u64] = &[728, 10316, 55013, 32803, 12281, 15100, 16645, 255];
/// A MagicTable stores 64 magic bitboards for each square in a board.
pub type MagicTable = [SuperMagic; 64];

/// A SuperMagic stores all necessary info for getting a magic bitboard for a square,
/// along with an offset, which allows for all of squares to be stored in a single array.
#[derive(Debug)]
pub struct SuperMagic {
    mask: Bitboard,
    magic: u64,
    shift: u8,
    offset: usize,
}

/// Stores individual magic bitboards.
pub struct Magic {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
}

pub struct MagicBitboard {
    magics: MagicTable,
    table: Vec<Bitboard>,
}

pub struct MagicBitboardSquare<'a> {
    magic: &'a SuperMagic,
    table: &'a [Bitboard],
}

pub enum MagicPiece {
    Bishop,
    Rook,
}

impl SuperMagic {
    pub fn new(mask: Bitboard, magic: u64, shift: u8, offset: usize) -> SuperMagic {
        SuperMagic {
            mask,
            magic,
            shift,
            offset,
        }
    }
}

impl MagicBitboard {
    pub fn new(magics: MagicTable, table: Vec<Bitboard>) -> MagicBitboard {
        MagicBitboard {
            magics,
            table,
        }
    }

    pub fn get<'a>(&'a self, square: Square) -> MagicBitboardSquare<'a> {
        MagicBitboardSquare {
            magic: &self.magics[square],
            table: &self.table,
        }
    }
}

impl Index<Bitboard> for MagicBitboardSquare<'_> {
    type Output = Bitboard;

    fn index(&self, index: Bitboard) -> &Self::Output {
        &self.table[self.magic.index(index)]
    }
}

impl Index<Square> for MagicTable {
    type Output = SuperMagic;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index as usize]
    }
}

impl MagicPiece {
    #[inline]
    pub fn ptype(&self) -> PieceType {
        match self {
            MagicPiece::Bishop => PieceType::Bishop,
            MagicPiece::Rook => PieceType::Rook,
        }
    }
}

impl SuperMagic {
    #[inline]
    pub fn index(&self, occupied: Bitboard) -> usize {
        self.offset + magic_index(self.magic, self.mask, self.shift, occupied)
    }
}

impl Magic {
    #[inline]
    pub fn index(&self, occupied: Bitboard) -> usize {
        magic_index(self.magic, self.mask, self.shift, occupied)
    }
}


