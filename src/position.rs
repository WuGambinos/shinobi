use crate::BitBoard;
use crate::Pieces;
use crate::Side;
use crate::Square;

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
}

#[derive(Debug, Clone)]
pub struct Position {
    /// Board for each side
    pub bitboard_sides: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub bitboard_pieces: [[BitBoard; 6]; 2],

    /// State contains all relveant information for evalution
    pub state: State,
}

impl Position {
    pub fn new() -> Position {
        Position {
            bitboard_sides: [BitBoard(0); 2],
            bitboard_pieces: [[BitBoard(0); 6]; 2],
            state: State::new(),
        }
    }

    pub fn from_grid(&mut self, grid: [char; 64]) {
        for (i, ch) in grid.iter().enumerate() {
            let mask = BitBoard(1u64 << i);

            let piece = match ch {
                'P' => Pieces::Pawn as usize,
                'B' => Pieces::Bishop as usize,
                'N' => Pieces::Knight as usize,
                'R' => Pieces::Rook as usize,
                'Q' => Pieces::Queen as usize,
                'K' => Pieces::King as usize,

                'p' => Pieces::Pawn as usize,
                'b' => Pieces::Bishop as usize,
                'n' => Pieces::Knight as usize,
                'r' => Pieces::Rook as usize,
                'q' => Pieces::Queen as usize,
                'k' => Pieces::King as usize,

                _ => 0,
            };

            if ch.is_ascii() {
                if ch.is_uppercase() {
                    self.bitboard_sides[Side::White as usize] |= mask;
                    self.bitboard_pieces[Side::White as usize][piece] |= mask;
                } else if ch.is_lowercase() {
                    self.bitboard_sides[Side::Black as usize] |= mask;
                    self.bitboard_pieces[Side::Black as usize][piece] |= mask;
                }
            }
        }
    }

    pub fn clear_square(&mut self, square: u64) {}

    pub fn make_move(&mut self) {
        let from_bitboard: BitBoard = BitBoard(1) << 11;
        let to_bitboard: BitBoard = BitBoard(1) << 19;
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Update piece bitboard
        self.bitboard_pieces[self.state.turn as usize][Pieces::Pawn as usize] ^= from_to_bitboard;

        // Update white or black bitboard
        self.bitboard_sides[self.state.turn as usize] ^= from_to_bitboard;
    }

    pub fn print_black_piece_bitboards(&self) {
        for (i, bitboard) in self.bitboard_pieces[Side::Black as usize]
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
            self.print_bitboard(*bitboard);
        }
    }

    pub fn print_bitboard(&self, bitboard: BitBoard) {
        let board = bitboard;
        println!("BOARD NUM: {}", board.0);
        println!("BINARY: {:#064b}", board.0);
        println!();
        println!("A B C D E F G H");
        for rank in 0..8 {
            for file in 0..8 {
                let square = rank * 8 + file;

                let bit = board.get_bit(square);

                if bit != 0 {
                    print!(" {} ", 1);
                } else {
                    print!(" {} ", 0);
                }
            }
            println!();
        }
    }

    pub fn print_black_bitboard(&self) {
        self.print_bitboard(self.bitboard_sides[Side::Black as usize]);
    }

    pub fn print_white_bitboard(&self) {
        self.print_bitboard(self.bitboard_sides[Side::White as usize]);
    }
}
