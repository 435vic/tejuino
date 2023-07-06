//! # Tejuino: A chess engine written in Rust
//! 
//! ## WARNING: VERY WORK IN PROGRESS

pub mod movegen;
pub mod types;
pub mod util;
pub mod pregen;
pub mod magic;

use pregen::Pregen;
use types::*;

use lazy_static::lazy_static;

pub const BOARD_START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

lazy_static! {
    pub static ref PREGEN: Pregen = Pregen::init();
}

// Tests
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_square() {
        let all_files = vec![File::FileA, File::FileB, File::FileC, File::FileD, File::FileE, File::FileF, File::FileG, File::FileH];
        let all_ranks = vec![Rank::Rank1, Rank::Rank2, Rank::Rank3, Rank::Rank4, Rank::Rank5, Rank::Rank6, Rank::Rank7, Rank::Rank8];

        for sq in Square::all() {
            let n = sq as usize;
            assert_eq!(Square::from(n as isize), sq);
            let file_n = n % 8;
            let rank_n = n / 8;
            assert_eq!(sq.file(), all_files[file_n]);
            assert_eq!(sq.rank(), all_ranks[rank_n]);
        }

        // square methods
        assert_eq!(Square::E4.dist(Square::E4), 0);
        assert_eq!(Square::E4.dist(Square::E5), 1);

        struct KnightJump {
            from: Square,
            to: Square,
        }

        impl KnightJump {
            fn new(from: Square, to: Square) -> KnightJump {
                KnightJump { from, to }
            }
        }

        let knight_jumps = vec![
            KnightJump::new(Square::E4, Square::C3),
            KnightJump::new(Square::E4, Square::G3),
            KnightJump::new(Square::E4, Square::C5),
            KnightJump::new(Square::E4, Square::G5),
            KnightJump::new(Square::E4, Square::D2),
            KnightJump::new(Square::E4, Square::F2),
            KnightJump::new(Square::E4, Square::D6),
            KnightJump::new(Square::E4, Square::F6),
            KnightJump::new(Square::G1, Square::F3),
            KnightJump::new(Square::H8, Square::F7),
        ];

        for jmp in knight_jumps {
            println!("{} -> {}", jmp.from, jmp.to);
            assert_eq!(jmp.from.dist(jmp.to), 2);
        }
    }

    #[test]
    fn test_pregen() {
        assert_eq!(
            PREGEN.attacks.king[Square::E4],
            Bitboard::squares(&[Square::D3, Square::E3, Square::F3, Square::D4,
                                Square::F4, Square::D5, Square::E5, Square::F5])
        );

        assert_eq!(
            PREGEN.attacks.knight[Square::E4],
            Bitboard::squares(&[Square::C3, Square::G3, Square::C5, Square::G5,
                                Square::D2, Square::F2, Square::D6, Square::F6])
        )
    }

    #[test]
    fn test_edges() {
        let s = Square::A1;
        let b = ((Bitboard::FILE_A | Bitboard::FILE_H) & !Bitboard::file(s)) | ((Bitboard::RANK_1 | Bitboard::RANK_8) & !Bitboard::rank(s));
        println!("{:?}", b);
    }
}
