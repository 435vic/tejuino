use crate::util;
use crate::Piece;

pub struct Board {
    content: [Piece; 64]
}

impl Board {
    pub fn new() -> Board {
        Board {
            content: [Piece::None; 64]
        }
    }

    pub fn from(fen: &str) -> Board {
        let mut _board = Board::new();
        let chunks: Vec<&str> = fen.split(' ').collect();
        let mut file: usize = 0;
        let mut rank: usize = 0;
        for c in chunks[0].chars() {
            match c {
                '/' => {
                    file = 0;
                    rank += 1;
                    continue;
                }
                c if c.is_numeric() => {
                    let num: u32 = c.to_digit(10).expect("Failed type conversion.");
                    file += usize::try_from(num).unwrap();
                }
                _ => {
                    _board.content[rank*8+file] = Piece::from(c);
                    file += 1;
                }
            }
        }
        _board
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}

// implement the debug display trait for Board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Board] to be implemented")
    }
}

// implement the display trait for Board
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.content.iter().map(|p| p.id()).collect::<Vec<char>>();
        let chars_slice: &[char; 64] = chars.as_slice()
            .try_into().expect("&self.content should be of length 64.");
        write!(f, "{}", util::render_grid(chars_slice))
    }
}
