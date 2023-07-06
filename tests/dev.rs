#[cfg(test)]
mod tests {
    use tejuino::types::*;
    use tejuino::movegen::*;

    #[test]
    fn test_print() {
        let my_board = Board::from(tejuino::BOARD_START_FEN);
        my_board.print();
        println!("{:?}\n{:?}\n{:?}", 
            my_board.pieces[PieceType::All as usize],
            my_board.colors[Color::White as usize],
            my_board.colors[Color::Black as usize]
        );
    }

    #[test]
    fn bitboard_test_print() {
        let b = Bitboard(0x0FF0);
        println!("{:?}", b);
        for sq in b.iter() {
            println!("{:?}", sq);
        }
    }

    #[test]
    fn test_sliding_attack() {
        let occupied = Bitboard::squares(&[Square::B5, Square::F5, Square::D2, Square::D7, Square::D5]);
        let attack_bb = sliding_attack(
            PieceType::Rook,
            Square::D5,
            occupied
        );
        println!("{:?}", attack_bb);
        println!("{:?}", occupied);
    }

    #[test]
    fn test_attack() {
        let board = Board::from("rnbqkbnr/p1pppppp/8/2B5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
        let square = Square::C5;
        let attack_bb = attack(
            board.at(square),
            square,
            board.pieces[PieceType::All as usize]
        ) & !board.colors[Color::White as usize];
        println!("{:?}", attack_bb);
    }
}
