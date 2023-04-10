use crate::BitBoard;
use crate::Piece;
use crate::SMagic;
use crate::Side;
use crate::Square;
use crate::SquareLabel;
use crate::A_FILE;
use crate::B_FILE;
use crate::G_FILE;
use crate::H_FILE;
use strum::IntoEnumIterator;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    fn enemy(&mut self) -> Side {
        match self.turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    // BitBoard that shows combined states of white and black bitboards
    pub main_bitboard: BitBoard,

    // BitBoard showing which squares are empty
    pub empty_bitboard: BitBoard,

    /// Board for each side
    pub side_bitboards: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub piece_bitboards: [[BitBoard; 6]; 2],

    // BitBoard for piece attacks
    //pub attack_bitboards: [[BitBoard; 6]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub pawn_pushes: [[BitBoard; 64]; 2],
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub king_attacks: [BitBoard; 64],

    pub bishop_attacks: [[BitBoard; 512]; 64],
    pub rook_attacks: [[BitBoard; 4096]; 64],

    pub bishop_tbl: [SMagic; 64],
    pub rook_tbl: [SMagic; 64],

    /// State contains all relveant information for evalution
    pub state: State,
}

impl Position {
    pub fn new() -> Position {
        Position {
            main_bitboard: BitBoard(0),
            empty_bitboard: BitBoard(u64::MAX),
            side_bitboards: [BitBoard(0); 2],
            piece_bitboards: [[BitBoard(0); 6]; 2],
            //attack_bitboards: [[BitBoard(0); 6]; 2],
            knight_attacks: [BitBoard(0); 64],
            pawn_pushes: [[BitBoard(0); 64]; 2],
            pawn_attacks: [[BitBoard(0); 64]; 2],
            king_attacks: [BitBoard(0); 64],

            bishop_attacks: [[BitBoard(0); 512]; 64],
            rook_attacks: [[BitBoard(0); 4096]; 64],

            bishop_tbl: [SMagic::new(0, 0); 64],
            rook_tbl: [SMagic::new(0, 0); 64],
            state: State::new(),
        }
    }

    pub fn from_grid(&mut self, grid: [char; 64]) {
        for (i, ch) in grid.iter().enumerate() {
            let mask = BitBoard(1u64 << i);

            let piece = match ch {
                'P' | 'p' => Piece::Pawn as usize,
                'B' | 'b' => Piece::Bishop as usize,
                'N' | 'n' => Piece::Knight as usize,
                'R' | 'r' => Piece::Rook as usize,
                'Q' | 'q' => Piece::Queen as usize,
                'K' | 'k' => Piece::King as usize,
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
                } else {
                    self.empty_bitboard |= mask;
                }
            }
        }
    }

    pub fn get_piece_on_square(&self, square: SquareLabel, side: Side) -> Option<Piece> {
        let pieces = self.piece_bitboards[side as usize];

        for piece in Piece::iter() {
            let board = pieces[piece as usize];

            if board.get_bit(square as u64) == 1 {
                return Some(piece);
            }
        }
        return None;
    }

    pub fn make_move(&mut self, piece: Piece, from_square: SquareLabel, to_square: SquareLabel) {
        let from_bitboard: BitBoard = BitBoard(1) << (from_square as usize);
        let to_bitboard: BitBoard = BitBoard(1) << (to_square as usize);
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let enemy: Side = self.state.enemy();

        if from_square != to_square
            && self.side_bitboards[self.state.turn as usize].get_bit(to_square as u64) == 0
        {
            // Check from_square has piece on it
            if self.side_bitboards[self.state.turn as usize].get_bit(from_square as u64) != 0 {
                if self.side_bitboards[self.state.enemy() as usize].get_bit(to_square as u64) == 1 {
                    // Update piece bitboard
                    self.piece_bitboards[self.state.turn as usize][piece as usize] ^=
                        from_to_bitboard;

                    // Update white or black bitboard
                    self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                    let enemy_piece = self.get_piece_on_square(to_square, enemy).unwrap();

                    // Reset captured piece
                    self.piece_bitboards[enemy as usize][enemy_piece as usize] ^= to_bitboard;

                    // Update color bitboard for captured side
                    self.side_bitboards[enemy as usize] ^= to_bitboard;

                    // Update main_bitboard
                    self.main_bitboard ^= from_to_bitboard;
                } else {
                    // Update piece bitboard
                    self.piece_bitboards[self.state.turn as usize][piece as usize] ^=
                        from_to_bitboard;

                    // Update white or black bitboard
                    self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                    // Update main_bitboard
                    self.main_bitboard ^= from_to_bitboard;
                }

                self.state.change_turn();
            }
        }
    }

    pub fn set_bit_on_piece_bitboard(&mut self, piece: Piece, side: Side, square: SquareLabel) {
        self.piece_bitboards[side as usize][piece as usize].set_bit(square);
    }

    pub fn north_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard << 8;
    }

    pub fn south_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard >> 8;
    }

    pub fn east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 1) & BitBoard(!A_FILE);
    }

    pub fn west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 1) & BitBoard(!H_FILE);
    }

    pub fn north_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 9) & BitBoard(!A_FILE);
    }

    pub fn north_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 7) & BitBoard(!H_FILE);
    }

    pub fn south_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 7) & BitBoard(!(A_FILE));
    }

    pub fn south_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 9) & BitBoard(!(H_FILE));
    }

    pub fn white_single_push_target(&self, bitboard: BitBoard) -> BitBoard {
        return self.north_one(bitboard) & self.empty_bitboard;
    }

    pub fn north_north_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 17) & BitBoard(!(A_FILE));
    }
    pub fn north_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 10) & BitBoard(!(A_FILE | B_FILE));
    }
    pub fn south_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 6) & BitBoard(!(A_FILE | B_FILE));
    }
    pub fn south_south_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 15) & BitBoard(!(A_FILE));
    }
    pub fn north_north_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 15) & BitBoard(!(H_FILE));
    }
    pub fn north_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 6) & BitBoard(!(G_FILE | H_FILE));
    }
    pub fn south_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 10) & BitBoard(!(G_FILE | H_FILE));
    }
    pub fn south_south_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 17) & BitBoard(!(H_FILE));
    }

    pub fn white_double_push_target(&self, bitboard: BitBoard) -> BitBoard {
        const RANK4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
        let single_pushes = self.white_single_push_target(bitboard);
        return self.north_one(single_pushes) & self.empty_bitboard & RANK4;
    }

    pub fn black_single_push_target(&self, bitboard: BitBoard) -> BitBoard {
        return self.south_one(bitboard) & self.empty_bitboard;
    }

    pub fn black_double_push_target(&self, bitboard: BitBoard) -> BitBoard {
        const RANK5: BitBoard = BitBoard(0x0000_00FF_0000_0000);
        let single_pushes = self.black_single_push_target(bitboard);
        return self.south_one(single_pushes) & self.empty_bitboard & RANK5;
    }

    pub fn generate_pawn_pushes(&self, side: Side, square: SquareLabel) -> BitBoard {
        let mut pushes: BitBoard = BitBoard(0);

        match side {
            Side::White => {
                let mut white_pawns: BitBoard = BitBoard(0);
                white_pawns.set_bit(square);
                pushes |= self.white_single_push_target(white_pawns);
                pushes |= self.white_double_push_target(white_pawns);
                return pushes;
            }
            Side::Black => {
                let mut black_pawns: BitBoard = BitBoard(0);
                black_pawns.set_bit(square);
                pushes |= self.black_single_push_target(black_pawns);
                pushes |= self.black_double_push_target(black_pawns);
                return pushes;
            }
        };
    }

    pub fn white_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.north_east_one(bitboard);
    }

    pub fn white_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.north_west_one(bitboard);
    }

    pub fn black_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.south_east_one(bitboard);
    }

    pub fn black_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        return self.south_west_one(bitboard);
    }

    pub fn generate_pawn_attacks(&self, side: Side, square: SquareLabel) -> BitBoard {
        let mut attacks: BitBoard = BitBoard(0);
        let mut bitboard: BitBoard = BitBoard(0);

        bitboard.set_bit(square);

        match side {
            Side::White => {
                attacks |= self.white_pawn_east_attacks(bitboard);
                attacks |= self.white_pawn_west_attacks(bitboard);
            }
            Side::Black => {
                attacks |= self.black_pawn_east_attacks(bitboard);
                attacks |= self.black_pawn_west_attacks(bitboard);
            }
        }

        attacks
    }

    pub fn generate_king_moves(&self, square: SquareLabel) -> BitBoard {
        let mut attacks: BitBoard = BitBoard(0);
        let mut bitboard: BitBoard = BitBoard(0);

        bitboard.set_bit(square);

        attacks = self.east_one(bitboard) | self.west_one(bitboard);
        bitboard |= attacks;
        attacks |= self.north_one(bitboard) | self.south_one(bitboard);

        return attacks;
    }

    pub fn generate_pawn_moves(&self, side: Side, square: SquareLabel) -> BitBoard {
        self.generate_pawn_pushes(side, square) | self.generate_pawn_attacks(side, square)
    }

    pub fn generate_knight_moves(&self, square: SquareLabel) -> BitBoard {
        let mut attacks: BitBoard = BitBoard(0);
        let mut bitboard: BitBoard = BitBoard(0);

        bitboard.set_bit(square);

        attacks |= self.north_north_east(bitboard);
        attacks |= self.north_east_east(bitboard);
        attacks |= self.south_east_east(bitboard);
        attacks |= self.south_south_east(bitboard);
        attacks |= self.north_north_west(bitboard);
        attacks |= self.north_west_west(bitboard);
        attacks |= self.south_west_west(bitboard);
        attacks |= self.south_south_west(bitboard);

        attacks
    }

    pub fn geneate_biship_moves(&self, occ: u64, sqaure: SquareLabel) {
    }

    pub fn generate_moves(&mut self) {
        for square in SquareLabel::iter() {
            self.knight_attacks[square as usize] = self.generate_knight_moves(square);
            self.pawn_pushes[Side::White as usize][square as usize] =
                self.generate_pawn_moves(Side::White, square);
            self.pawn_pushes[Side::Black as usize][square as usize] =
                self.generate_pawn_moves(Side::Black, square);

            self.king_attacks[square as usize] = self.generate_knight_moves(square);
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
