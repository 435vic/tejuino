use crate::types::*;
use crate::PREGEN;

const ROOK_DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Right, Direction::Down, Direction::Left];
const BISHOP_DIRECTIONS: &[Direction] = &[Direction::UpRight, Direction::UpLeft, Direction::DownRight, Direction::DownLeft];

pub fn sliding_attack(ptype: PieceType, sq: Square, occupied: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = Bitboard(0);
    let directions = match ptype {
        PieceType::Rook => ROOK_DIRECTIONS,
        PieceType::Bishop => BISHOP_DIRECTIONS,
        _ => return Bitboard(0),
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
    attacks
}

#[inline]
pub fn get_moves(piece: Piece, sq: Square, occupied: Bitboard) -> Bitboard {
    match piece.ptype() {
        PieceType::Knight => PREGEN.attacks.knight[sq],
        PieceType::King => PREGEN.attacks.king[sq],
        PieceType::Rook => PREGEN.rook_magics.get(sq)[occupied],
        PieceType::Bishop => PREGEN.bishop_magics.get(sq)[occupied],
        PieceType::Queen => PREGEN.rook_magics.get(sq)[occupied] | PREGEN.bishop_magics.get(sq)[occupied],
        _ => Bitboard(0)
    }
}

pub fn get_pawn_moves(sq: Square, ctx: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let us = ctx.side_to_move;
    let them = !us;
    let start_rank = match us {
        Color::White => Bitboard::RANK_2,
        Color::Black => Bitboard::RANK_7,
    };

    let double_pawns = ctx.by_piece(Piece::Pawn(us)) & start_rank;
    let single_pawns = ctx.by_piece(Piece::Pawn(us)) & !start_rank;

    let single_moves = single_pawns.shift(Direction::Up, 1) & !ctx.pieces();
    let double_moves = single_pawns.shift(Direction::Up, 2) & !ctx.pieces();

    moves
}

pub fn pseudolegal_moves(board: &Board) -> Vec<Move> {
    let mut moves = vec![];
    for sq in board.colors[board.side_to_move as usize].iter() {
        let piece = board.at(sq);
        let attacks = get_moves(piece, sq, board.pieces[PieceType::All as usize])
            & !board.colors[board.side_to_move as usize];
        for to in attacks.iter() {
            moves.push(Move::new(sq, to, MoveType::Quiet));
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use crate::types::Board;
    use crate::movegen::*;

    #[test]
    fn test_pseudo_gen() {
        let board = Board::from(crate::BOARD_START_FEN);
        let moves = pseudolegal_moves(&board);
        for m in moves {
            println!("{}", m);
        }
    }
}
