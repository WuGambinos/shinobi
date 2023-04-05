use crate::BitBoard;
use crate::Pieces;
use crate::Side;
use crate::Square;
use crate::SquareLabels;

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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_counter: u8,
    pub turn: Side,
}

impl State {
    fn new() -> State {
        State {
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            half_move_counter: 0,
            turn: Side::White,
        }
    }

    fn change_turn(&mut self) {
        match self.turn {
            Side::White => self.turn = Side::Black,
            Side::Black => self.turn = Side::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    // BitBoard that shows combined states of white and black bitboards
    pub main_bitboard: BitBoard,

    /// Board for each side
    pub side_bitboards: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub piece_bitboards: [[BitBoard; 6]; 2],

    /// State contains all relveant information for evalution
    pub state: State,
}

impl Position {
    pub fn new() -> Position {
        Position {
            main_bitboard: BitBoard(0),
            side_bitboards: [BitBoard(0); 2],
            piece_bitboards: [[BitBoard(0); 6]; 2],
            state: State::new(),
        }
    }

    pub fn from_grid(&mut self, grid: [char; 64]) {
        for (i, ch) in grid.iter().enumerate() {
            let mask = BitBoard(1u64 << i);

            let piece = match ch {
                'P' | 'p' => Pieces::Pawn as usize,
                'B' | 'b' => Pieces::Bishop as usize,
                'N' | 'n' => Pieces::Knight as usize,
                'R' | 'r' => Pieces::Rook as usize,
                'Q' | 'q' => Pieces::Queen as usize,
                'K' | 'k' => Pieces::King as usize,
                _ => 0,
            };

            if ch.is_ascii() {
                if ch.is_uppercase() {
                    self.side_bitboards[Side::White as usize] |= mask;
                    self.piece_bitboards[Side::White as usize][piece] |= mask;
                    self.main_bitboard |= mask;
                } else if ch.is_lowercase() {
                    self.side_bitboards[Side::Black as usize] |= mask;
                    self.piece_bitboards[Side::Black as usize][piece] |= mask;
                    self.main_bitboard |= mask;
                }
            }
        }
    }

    pub fn make_move(&mut self, from_square: SquareLabels, to_square: SquareLabels) {
        let from_bitboard: BitBoard = BitBoard(1) << (from_square as usize);
        let to_bitboard: BitBoard = BitBoard(1) << (to_square as usize);
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        if from_square != to_square {
            if self.state.turn == Side::White {
                if self.side_bitboards[Side::White as usize].get_bit(from_square as u64) != 0 {
                    // Update piece bitboard
                    self.piece_bitboards[self.state.turn as usize][Pieces::Pawn as usize] ^=
                        from_to_bitboard;

                    // Update white or black bitboard
                    self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                    // Update main_bitboard
                    self.main_bitboard ^= from_to_bitboard;

                    self.state.change_turn();
                }
            } else {
                if self.side_bitboards[Side::Black as usize].get_bit(from_square as u64) != 0 {
                    // Update piece bitboard
                    self.piece_bitboards[self.state.turn as usize][Pieces::Pawn as usize] ^=
                        from_to_bitboard;

                    // Update white or black bitboard
                    self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                    // Update main_bitboard
                    self.main_bitboard ^= from_to_bitboard;

                    self.state.change_turn();
                }
            }
        }
    }

    pub fn print_black_piece_bitboards(&self) {
        for (i, bitboard) in self.piece_bitboards[Side::Black as usize]
            .iter()
            .enumerate()
        {
            match i {
                0 => println!("PAWN"),
                1 => println!("BISHOP"),
                2 => println!("KNIGHT"),
                3 => println!("ROOK"),
                4 => println!("QUEEN"),
                5 => println!("KING"),
                _ => (),
            }
            //self.print_bitboard(*bitboard);
            bitboard.print();
        }
    }

    pub fn print_bitboard(&self, bitboard: BitBoard) {
        bitboard.print();
    }

    pub fn print_black_bitboard(&self) {
        self.side_bitboards[Side::Black as usize].print();
    }

    pub fn print_white_bitboard(&self) {
        self.side_bitboards[Side::White as usize].print();
    }
}
