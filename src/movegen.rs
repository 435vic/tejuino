use crate::types::{Direction, PieceType, Bitboard, Square};

const ROOK_DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Right, Direction::Down, Direction::Left];
const BISHOP_DIRECTIONS: &[Direction] = &[Direction::UpRight, Direction::UpLeft, Direction::DownRight, Direction::DownLeft];

pub fn sliding_attack(ptype: PieceType, sq: Square, occupied: Bitboard) -> Result<Bitboard, String> {
    let mut attacks: Bitboard = Bitboard(0);
    let directions = match ptype {
        PieceType::Rook => ROOK_DIRECTIONS,
        PieceType::Bishop => BISHOP_DIRECTIONS,
        _ => return Err(format!("Invalid piece type: {}", ptype)),
    };
    for direction in directions {
        let mut to = sq;
        while to.safe_step(*direction as isize) {
            to += *direction as isize;
            attacks |= Bitboard::square(to);
            if !(Bitboard::square(to) & occupied).empty() {
                break;
            }
        }
    }
    Ok(attacks)
}

