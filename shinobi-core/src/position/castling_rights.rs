
pub struct Castling(u8);
impl Castling {
    pub const WHITE_KING_SIDE: u8 = 0b1000;
    pub const WHITE_QUEEN_SIDE: u8 = 0b0100;
    pub const BLACK_KING_SIDE: u8 = 0b0010;
    pub const BLACK_QUEEN_SIDE: u8 = 0b0001;

    pub const WHITE_CASTLING: u8 = Self::WHITE_KING_SIDE | Self::WHITE_QUEEN_SIDE;
    pub const BLACK_CASTLING: u8 = Self::BLACK_KING_SIDE | Self::BLACK_QUEEN_SIDE;

    pub const NO_CASTLING: u8 = 0b0000;
    pub const ANY_CASTLING: u8 = Self::WHITE_CASTLING | Self::BLACK_CASTLING;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub fn empty() -> CastlingRights {
        CastlingRights(Castling::NO_CASTLING)
    }

    pub fn all() -> CastlingRights {
        CastlingRights(Castling::ANY_CASTLING)
    }

    pub fn white_king_side(self) -> bool {
        self.0 & Castling::WHITE_KING_SIDE != 0
    }

    pub fn white_queen_side(self) -> bool {
        self.0 & Castling::WHITE_QUEEN_SIDE != 0
    }

    pub fn black_king_side(self) -> bool {
        self.0 & Castling::BLACK_KING_SIDE != 0
    }

    pub fn black_queen_side(self) -> bool {
        self.0 & Castling::BLACK_QUEEN_SIDE != 0
    }
}
