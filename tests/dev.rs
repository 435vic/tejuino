#[cfg(test)]
mod tests {
    use tejuino::types::*;
    use tejuino::movegen::*;

    #[test]
    fn test_print() {
        let my_board = Board::from(tejuino::BOARD_START_FEN);
        my_board.print();
    }

    #[test]
    fn bitboard_test_print() {
        println!("{:?}", Bitboard(0xFF));
    }

    #[test]
    fn test_sliding_attack() {
        let occupied = Bitboard::squares(&[Square::B5, Square::F5, Square::D2, Square::D7]);
        let attack_bb = sliding_attack(
            PieceType::Rook,
            Square::D5,
            occupied
        );
        println!("{:?}", attack_bb.unwrap());
        println!("{:?}", occupied);
    }
}
