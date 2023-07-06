//! # Magic Bitboards
//! ## 'cause regular bitboards aren't magic enough
//! 
//! ### What are they?
//! Magic bitboards are a method of generating the attacks for sliding pieces
//! (rooks, bishops, queens) using a lookup table. Normally, to calculate them you'd iterate over
//! 'rays' starting from the piece's square, stopping when you hit a piece or the edge of the board.
//! However, this method is too slow for chess engines, as they have to process and generate
//! hundreds of thousands of moves a second! With magic bitboards, we basically offload the task
//! of calculating these rays and attacks at startup, and we look them up at runtime instead.
//! 
//! ### How do they work??
//! Magic bitboards are glorified hash tables. We 'hash' the blockers (pieces that block the
//! sliding piece) using a magic number (which we'll get into in a bit), which gives us an index
//! which we can use to store and later look up the corresponding attack squares. Each square on
//! the board has its own hash table, for a total of 64 hash tables for each sliding piece. In this
//! case, we can just store one for a rook and bishop, as the queen is a combination of both.
//! 
//! ### What is all this magic mumbo jumbo?
//! If we were to just use the blockers bitboard as an array index, it would be too big. We need
//! to find a way to 'hash' it so that it takes up less space. This is where the magic number comes
//! into play. We guess a random number, multiply it by the bitboard, and shift the result to the
//! right, so that we have less total bits. Of course, this random number might not work the first
//! time, creating hash collisions. In that case, we just try again with a different number.
//! 
//! If you still don't understand anything, I get it. I'm pretty bad at explaining stuff, so
//! you can also look at the links I left down below. Don't worry, I won't get jealous.
//! 
//! ### Links
//! - [Chess Programming Wiki](https://www.chessprogramming.org/Magic_Bitboards)
//! - [analog-hors](https://analog-hors.github.io/site/magic-bitboards/)
//! - [Stockfish implementation](https://github.com/official-stockfish/Stockfish/blob/e699fee513ce26b3794ac43d08826c89106e10ea/src/bitboard.cpp#L142)
//! 


use crate::types::*;
use crate::util::PRNG;
use crate::movegen::sliding_attack;

pub fn magic_index(magic: u64, mask: Bitboard, shift: u8, occupied: Bitboard) -> usize {
    let blockers = occupied & mask;
    let hash = blockers.0.wrapping_mul(magic);
    (hash >> shift) as usize
}

pub fn generate_magics(piece: &MagicPiece) -> MagicBitboard {
    let mut super_table: Vec<Bitboard> = vec![];
    let mut magics: Vec<SuperMagic> = vec![];
    let mut offset = 0;
    for sq in Square::all() {
        // RNG seeds shamelessly stolen from Stockfish
        let (magic, table) = find_magic(piece, sq, &mut PRNG::new(RNG_SEEDS[sq.rank() as usize]));
        magics.push(SuperMagic::new(magic.mask, magic.magic, magic.shift, offset));
        offset += table.len();
        super_table.extend(table);
    }
    assert_eq!(magics.len(), 64);
    MagicBitboard::new(magics.try_into().unwrap(), super_table)
}

// Magic bitboards implementation
// Largely based on https://www.chessprogramming.org/Magic_Bitboards
// Some code chunks and optimizations taken from stockfish
// Rust-specific details based on/inspired by cozy-chess by analog-hors
// https://github.com/analog-hors/cozy-chess

pub fn find_magic(
    piece: &MagicPiece,
    square: Square,
    rng: &mut PRNG,
) -> (Magic, Vec<Bitboard>) {
    let blockers = sliding_attack(piece.ptype(), square, Bitboard(0));
    let mask = blockers & !Bitboard::edges(square);
    let index_bits = mask.0.count_ones();
    let mut table = vec![Bitboard(0); 1 << index_bits];
    let mut m = Magic {
        mask,
        magic: 0,
        shift: 64 - index_bits as u8,
    };

    // Store every subset of blockers along with the attack squares, computed with sliding_attack.
    let mut perms: Vec<(Bitboard, Bitboard)> = vec![(Bitboard(0), Bitboard(0)); 1 << index_bits];

    let mut blockers_subset = Bitboard(0);
    for i in 0..(1 << index_bits) {
        let attack = sliding_attack(piece.ptype(), square, blockers_subset);
        perms[i] = (blockers_subset, attack);

        blockers_subset.0 = m.mask.0 & blockers_subset.0.wrapping_sub(m.mask.0);
        if blockers_subset.empty() {
            break;
        }
    }
    // sanity check
    assert!(blockers_subset.empty());

    // Keep track of attempts
    let mut epoch = vec![0; 1 << index_bits];
    let mut attempt_no = 0;

    // With all the subsets and their attacks computed, we can brute force the magic
    loop {
        // Magic calculation from stockfish (bitboard.cpp line 196)
        // generate magic until it has less than 6 ones in the most significant bits
        m.magic = 0;
        while (m.mask.0.wrapping_mul(m.magic) >> 56).count_ones() < 6 {
            m.magic = rng.sparse();
        }

        let mut success = true;

        attempt_no += 1;
        for i in 0..(1 << index_bits) {
            let (subset, attack) = perms[i];
            let idx = m.index(subset);
            // Previous attempt is stored, can be safely ignored
            if epoch[idx] < attempt_no {
                epoch[idx] = attempt_no; // Mark as 'visited'
                table[idx] = attack; // Store attack squares
            } else if table[idx] != attack {
                // Hash conflict, try with new magic.
                success = false;
                break;
            }
        }

        if success {
            break;
        }
    }
    (m, table)
}

#[cfg(test)]
mod tests {
    use crate::util::PRNG;
    use crate::types::*;
    use crate::magic::*;

    #[test]
    fn test_magic() {
        let mut rng = PRNG::new(0x891750FBADEA5);
        let (m, table) = find_magic(&MagicPiece::Rook, Square::B5, &mut rng);
        let blockers = Bitboard::squares(&[Square::F5, Square::B2, Square::B6]);
        println!("{:?}", table[m.index(Bitboard(0))]);
        println!("{:?}", table[m.index(blockers)]);

        let magic_bitboard = generate_magics(&MagicPiece::Rook);
        let magic = magic_bitboard.get(Square::B5);
        println!("{:?}", magic[blockers]);

        assert_eq!(table[m.index(Bitboard(0))], magic[Bitboard(0)]);
        assert_eq!(table[m.index(blockers)], magic[blockers]);
    }
}