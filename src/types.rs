pub mod piece;
pub mod square;
pub mod bitboard;
pub mod board;
pub mod magic;

use std::fmt::{Display, Debug};

pub use piece::*;
pub use square::*;
pub use bitboard::*;
pub use board::*;
pub use magic::*;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up = 8,
    Right = 1,
    Down = -8,
    Left = -1,

    UpRight = 8 + 1,
    UpLeft = 8 - 1,
    DownRight = -8 + 1,
    DownLeft = -8 - 1,
}

pub enum MoveType {
    Quiet,
    Promotion,
    EnPassant,
    Castle
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}


impl File {
    fn try_from(file: usize) -> Result<File, String> {
        match file {
            0 => Ok(File::FileA),
            1 => Ok(File::FileB),
            2 => Ok(File::FileC),
            3 => Ok(File::FileD),
            4 => Ok(File::FileE),
            5 => Ok(File::FileF),
            6 => Ok(File::FileG),
            7 => Ok(File::FileH),
            _ => Err(format!("Invalid file: {}", file)),
        }
    }
}

impl Rank {
    fn try_from(rank: usize) -> Result<Rank, String> {
        match rank {
            0 => Ok(Rank::Rank1),
            1 => Ok(Rank::Rank2),
            2 => Ok(Rank::Rank3),
            3 => Ok(Rank::Rank4),
            4 => Ok(Rank::Rank5),
            5 => Ok(Rank::Rank6),
            6 => Ok(Rank::Rank7),
            7 => Ok(Rank::Rank8),
            _ => Err(format!("Invalid rank: {}", rank)),
        }
    }
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Move {
    pub fn new(from: Square, to: Square, move_type: MoveType) -> Move {
        Move { from, to, move_type }
    }

    pub fn print(&self) {
        println!("{} -> {}", self.from, self.to);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
