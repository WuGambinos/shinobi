use crate::EnumIter;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq)]
pub enum Pieces {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SquareLabels {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<u64> for SquareLabels {
    fn from(value: u64) -> SquareLabels {
        match value {
            0 => SquareLabels::A1,
            1 => SquareLabels::B1,
            2 => SquareLabels::C1,
            3 => SquareLabels::D1,
            4 => SquareLabels::E1,
            5 => SquareLabels::F1,
            6 => SquareLabels::G1,
            7 => SquareLabels::H1,
            8 => SquareLabels::A2,
            9 => SquareLabels::B2,
            10 => SquareLabels::C2,
            11 => SquareLabels::D2,
            12 => SquareLabels::E2,
            13 => SquareLabels::F2,
            14 => SquareLabels::G2,
            15 => SquareLabels::H2,
            16 => SquareLabels::A3,
            17 => SquareLabels::B3,
            18 => SquareLabels::C3,
            19 => SquareLabels::D3,
            20 => SquareLabels::E3,
            21 => SquareLabels::F3,
            22 => SquareLabels::G3,
            23 => SquareLabels::H3,
            24 => SquareLabels::A4,
            25 => SquareLabels::B4,
            26 => SquareLabels::C4,
            27 => SquareLabels::D4,
            28 => SquareLabels::E4,
            29 => SquareLabels::F4,
            30 => SquareLabels::G4,
            31 => SquareLabels::H4,
            32 => SquareLabels::A5,
            33 => SquareLabels::B5,
            34 => SquareLabels::C5,
            35 => SquareLabels::D5,
            36 => SquareLabels::E5,
            37 => SquareLabels::F5,
            38 => SquareLabels::G5,
            39 => SquareLabels::H5,
            40 => SquareLabels::A6,
            41 => SquareLabels::B6,
            42 => SquareLabels::C6,
            43 => SquareLabels::D6,
            44 => SquareLabels::E6,
            45 => SquareLabels::F6,
            46 => SquareLabels::G6,
            47 => SquareLabels::H6,
            48 => SquareLabels::A7,
            49 => SquareLabels::B7,
            50 => SquareLabels::C7,
            51 => SquareLabels::D7,
            52 => SquareLabels::E7,
            53 => SquareLabels::F7,
            54 => SquareLabels::G7,
            55 => SquareLabels::H7,
            56 => SquareLabels::A8,
            57 => SquareLabels::B8,
            58 => SquareLabels::C8,
            59 => SquareLabels::D8,
            60 => SquareLabels::E8,
            61 => SquareLabels::F8,
            62 => SquareLabels::G8,
            63 => SquareLabels::H8,
            _ => panic!("Invalid integer value for SquareLabels"),
        }
    }
}

/*
pub enum SquareLabels {
  A8, B8, C8, D8, E8, F8, G8, H8,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A1, B1, C1, D1, E1, F1, G1, H1,
}
*/
