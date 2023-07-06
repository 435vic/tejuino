use std::fmt::{Display, Debug};

use crate::util;
use crate::types::*;

pub struct Board {
    content: [Piece; 64],
    pub pieces: [Bitboard; 8],
    pub colors: [Bitboard; 2],
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Board {
        Board {
            content: [Piece::None; 64],
            pieces: [Bitboard(0); 8],
            colors: [Bitboard(0); 2],
            side_to_move: Color::White,
        }
    }

    pub fn from(fen: &str) -> Board {
        let mut _board = Board::new();
        let chunks: Vec<&str> = fen.split(' ').collect();
        let mut file: usize = 0;
        let mut rank: usize = 7;
        for c in chunks[0].chars() {
            match c {
                '/' => {
                    file = 0;
                    rank -= 1;
                    continue;
                }
                c if c.is_numeric() => {
                    let num: u32 = c.to_digit(10).expect("Failed type conversion.");
                    file += usize::try_from(num).unwrap();
                }
                _ => {
                    let piece = Piece::from(c);
                    let square_bb = Bitboard::square(Square::from(rank * 8 + file));
                    _board.content[rank * 8 + file] = piece;
                    _board.pieces[piece.ptype() as usize] |= square_bb;
                    _board.colors[piece.color().unwrap() as usize] |= square_bb;
                    _board.pieces[PieceType::All as usize] |= square_bb;
                    file += 1;
                }
            }
        }
        _board
    }

    #[inline]
    pub fn at(&self, sq: Square) -> Piece {
        self.content[sq as usize]
    }

    #[inline]
    pub fn by_color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    #[inline]
    pub fn by_piece_type(&self, ptype: PieceType) -> Bitboard {
        self.pieces[ptype as usize]
    }

    #[inline]
    pub fn by_piece(&self, piece: Piece) -> Bitboard {
        self.by_color(piece.color().unwrap()) & self.by_piece_type(piece.ptype())
    }

    #[inline]
    pub fn pieces(&self) -> Bitboard {
        self.by_piece_type(PieceType::All)
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}

// implement the debug display trait for Board
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Board] to be implemented")
    }
}

// implement the display trait for Board
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.content.iter().map(|p| p.id()).collect::<Vec<char>>();
        let chars_slice: &[char; 64] = chars
            .as_slice()
            .try_into()
            .expect("&self.content should be of length 64.");
        write!(f, "\n{}", util::render_grid(chars_slice, true))
    }
}
