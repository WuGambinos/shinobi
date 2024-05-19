use serde::{Deserialize, Serialize};

use crate::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Side {
    White,
    Black,
}

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

#[rustfmt::skip]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Square {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<&str> for Square {
    fn from(sq: &str) -> Square {
        let file = sq.chars().next().unwrap();
        let rank = sq.chars().nth(1).unwrap();

        let file_num = file as u8 - b'a';
        let rank_num = rank.to_digit(10).unwrap() as u8;

        let sq = (rank_num - 1) * 8 + file_num;

        Square::from(sq as u64)
    }
}

impl From<u64> for Square {
    fn from(value: u64) -> Square {
        match value {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            _ => panic!("Invalid integer value for SquareLabels {}", value),
        }
    }
}

impl From<u32> for Square {
    fn from(value: u32) -> Square {
        match value {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            _ => panic!("Invalid integer value for SquareLabels {}", value),
        }
    }
}
