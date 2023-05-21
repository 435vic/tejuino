pub mod board;
pub mod movegen;
pub mod types;
pub mod util;

use movegen::pregen::PseudoAttacks;
use types::*;

use lazy_static::lazy_static;

pub const BOARD_START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

lazy_static! {
    pub static ref PSEUDO_ATTACKS: PseudoAttacks = PseudoAttacks::init();
}

// Tests
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_square() {
        assert_eq!(Square::E4.file(), File::FileE);
        assert_eq!(Square::E4.rank(), Rank::Rank4);

        // square methods
        assert_eq!(Square::E4.dist(&Square::E4), 0);
        assert_eq!(Square::E4.dist(&Square::E5), 1);
    }

    #[test]
    fn pregen() {
        let pregen = movegen::pregen::PseudoAttacks::init();
        println!(
            "{:?}",
            pregen.king[Square::B5] | Bitboard::square(Square::B5)
        );
    }
}
