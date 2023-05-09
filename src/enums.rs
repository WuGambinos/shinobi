use crate::EnumIter;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
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
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SquareLabel {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<u64> for SquareLabel {
    fn from(value: u64) -> SquareLabel {
        match value {
            0 => SquareLabel::A1,
            1 => SquareLabel::B1,
            2 => SquareLabel::C1,
            3 => SquareLabel::D1,
            4 => SquareLabel::E1,
            5 => SquareLabel::F1,
            6 => SquareLabel::G1,
            7 => SquareLabel::H1,
            8 => SquareLabel::A2,
            9 => SquareLabel::B2,
            10 => SquareLabel::C2,
            11 => SquareLabel::D2,
            12 => SquareLabel::E2,
            13 => SquareLabel::F2,
            14 => SquareLabel::G2,
            15 => SquareLabel::H2,
            16 => SquareLabel::A3,
            17 => SquareLabel::B3,
            18 => SquareLabel::C3,
            19 => SquareLabel::D3,
            20 => SquareLabel::E3,
            21 => SquareLabel::F3,
            22 => SquareLabel::G3,
            23 => SquareLabel::H3,
            24 => SquareLabel::A4,
            25 => SquareLabel::B4,
            26 => SquareLabel::C4,
            27 => SquareLabel::D4,
            28 => SquareLabel::E4,
            29 => SquareLabel::F4,
            30 => SquareLabel::G4,
            31 => SquareLabel::H4,
            32 => SquareLabel::A5,
            33 => SquareLabel::B5,
            34 => SquareLabel::C5,
            35 => SquareLabel::D5,
            36 => SquareLabel::E5,
            37 => SquareLabel::F5,
            38 => SquareLabel::G5,
            39 => SquareLabel::H5,
            40 => SquareLabel::A6,
            41 => SquareLabel::B6,
            42 => SquareLabel::C6,
            43 => SquareLabel::D6,
            44 => SquareLabel::E6,
            45 => SquareLabel::F6,
            46 => SquareLabel::G6,
            47 => SquareLabel::H6,
            48 => SquareLabel::A7,
            49 => SquareLabel::B7,
            50 => SquareLabel::C7,
            51 => SquareLabel::D7,
            52 => SquareLabel::E7,
            53 => SquareLabel::F7,
            54 => SquareLabel::G7,
            55 => SquareLabel::H7,
            56 => SquareLabel::A8,
            57 => SquareLabel::B8,
            58 => SquareLabel::C8,
            59 => SquareLabel::D8,
            60 => SquareLabel::E8,
            61 => SquareLabel::F8,
            62 => SquareLabel::G8,
            63 => SquareLabel::H8,
            _ => panic!("Invalid integer value for SquareLabels {}", value),
        }
    }
}
