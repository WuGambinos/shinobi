use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::Side;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<u32> for Piece {
    fn from(p: u32) -> Piece {
        match p {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("NOT A PIECE: {}", p),
        }
    }
}

impl Piece {
    pub fn is_pawn(self) -> bool {
        self == Piece::Pawn
    }

    pub fn is_bishop(self) -> bool {
        self == Piece::Bishop
    }

    pub fn is_knight(self) -> bool {
        self == Piece::Knight
    }

    pub fn is_queen(self) -> bool {
        self == Piece::Queen
    }

    pub fn is_king(self) -> bool {
        self == Piece::King
    }

    pub fn is_rook(self) -> bool {
        self == Piece::Rook
    }

    pub fn is_slider(self) -> bool {
        self.is_rook() || self.is_queen() || self.is_bishop()
    }

    pub fn to_char(self, side: Side) -> char {
        if side == Side::White {
            match self {
                Piece::King => 'K',
                Piece::Rook => 'R',
                Piece::Queen => 'Q',
                Piece::Bishop => 'B',
                Piece::Knight => 'N',
                Piece::Pawn => 'P',
            }
        } else {
            match self {
                Piece::King => 'k',
                Piece::Rook => 'r',
                Piece::Queen => 'q',
                Piece::Bishop => 'b',
                Piece::Knight => 'n',
                Piece::Pawn => 'p',
            }
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Piece {
        match value {
            'k' | 'K' => Piece::King,
            'p' | 'P' => Piece::Pawn,
            'r' | 'R' => Piece::Rook,
            'q' | 'Q' => Piece::Queen,
            'b' | 'B' => Piece::Bishop,
            'n' | 'N' => Piece::Knight,
            _ => panic!("NOT A PIECE"),
        }
    }
}
