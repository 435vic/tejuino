#[cfg(test)]
mod tests {
    use tejuino::board::Board;
    use tejuino::types::Bitboard;

    #[test]
    fn test_print() {
        let my_board = Board::from(tejuino::BOARD_START_FEN);
        my_board.print();
        // my_board.dev_print();
    }

    #[test]
    fn bitboard_test_print() {
        println!("{:?}", Bitboard(0xFF));
    }
}
