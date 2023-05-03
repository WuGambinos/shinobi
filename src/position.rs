use std::fmt;

use crate::adjacent_files;
use crate::get_file;
use crate::get_rank;
use crate::square_name;
use crate::BitBoard;
use crate::MoveGenerator;
use crate::Piece;
use crate::Side;
use crate::Square;
use crate::SquareLabel;
use crate::EMPTY_BITBOARD;
use crate::E_FILE;
use crate::FIFTH_RANK;
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
    pub en_passant_square: Option<SquareLabel>,
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

    pub fn enemy(&self) -> Side {
        match self.turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum MoveType {
    EnPassant,
    Quiet,
    Capture,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub piece: Piece,
    pub from_square: SquareLabel,
    pub target_square: SquareLabel,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(
        piece: Piece,
        from_square: SquareLabel,
        target_square: SquareLabel,
        move_type: MoveType,
    ) -> Move {
        Move {
            piece,
            from_square,
            target_square,
            move_type,
        }
    }

    pub fn is_double_pawn_push(&self) -> bool {
        return self.piece.is_pawn()
            && (self.target_square() as i8 - self.from_square() as i8).abs() == 16;
    }

    fn from_square(&self) -> SquareLabel {
        self.from_square
    }

    fn target_square(&self) -> SquareLabel {
        self.target_square
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            square_name(self.from_square() as u8),
            square_name(self.target_square() as u8)
        )
    }
}

#[derive(Debug, Clone)]
pub struct History {
    pub moves: Vec<Move>,
    pub prev_pieces: Vec<[Option<(Side, Piece)>; 64]>,
    pub prev_main_bitboards: Vec<BitBoard>,
    pub prev_empty_bitboards: Vec<BitBoard>,
    pub prev_side_bitboards: Vec<[BitBoard; 2]>,
    pub prev_piece_bitboards: Vec<[[BitBoard; 6]; 2]>,
    pub prev_states: Vec<State>,
    pub prev_white_king_square: Option<SquareLabel>,
    pub prev_black_king_square: Option<SquareLabel>,
}

impl History {
    fn new() -> History {
        History {
            moves: Vec::new(),
            prev_pieces: Vec::new(),
            prev_main_bitboards: Vec::new(),
            prev_empty_bitboards: Vec::new(),
            prev_side_bitboards: Vec::new(),
            prev_piece_bitboards: Vec::new(),
            prev_states: Vec::new(),
            prev_white_king_square: None,
            prev_black_king_square: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub pieces: [Option<(Side, Piece)>; 64],
    // BitBoard that shows combined states of white and black bitboards
    pub main_bitboard: BitBoard,

    // BitBoard showing which squares are empty
    pub empty_bitboard: BitBoard,

    /// Board for each side
    pub side_bitboards: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub piece_bitboards: [[BitBoard; 6]; 2],

    pub move_gen: MoveGenerator,

    /// State contains all relveant information for evalution
    pub state: State,

    pub white_king_square: SquareLabel,
    pub black_king_square: SquareLabel,

    pub history: History,
    pub last_move: Option<Move>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            pieces: [None; 64],
            main_bitboard: EMPTY_BITBOARD,
            empty_bitboard: EMPTY_BITBOARD,
            side_bitboards: [EMPTY_BITBOARD; 2],
            piece_bitboards: [[EMPTY_BITBOARD; 6]; 2],
            move_gen: MoveGenerator::new(),

            state: State::new(),

            white_king_square: SquareLabel::A1,
            black_king_square: SquareLabel::A1,

            history: History::new(),
            last_move: None,
        }
    }

    pub fn enemy_bitboard(&self) -> BitBoard {
        return self.side_bitboards[self.state.enemy() as usize];
    }

    pub fn get_piece_bitboard(&self, piece: Piece, side: Side) -> BitBoard {
        self.piece_bitboards[side as usize][piece as usize]
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

            if *ch == 'K' {
                self.white_king_square = SquareLabel::from(i as u64);
            } else if *ch == 'k' {
                self.black_king_square = SquareLabel::from(i as u64);
            }

            if ch.is_ascii() {
                if ch.is_uppercase() {
                    self.side_bitboards[Side::White as usize] |= mask;
                    self.piece_bitboards[Side::White as usize][piece] |= mask;
                    self.main_bitboard |= mask;
                    self.pieces[i] = Some((Side::White, Piece::from(*ch)));
                } else if ch.is_lowercase() {
                    self.side_bitboards[Side::Black as usize] |= mask;
                    self.piece_bitboards[Side::Black as usize][piece] |= mask;
                    self.main_bitboard |= mask;
                    self.pieces[i] = Some((Side::Black, Piece::from(*ch)));
                } else {
                    self.empty_bitboard |= mask;
                }
            }
        }
    }

    pub fn get_king_square(&self, side: Side) -> Option<SquareLabel> {
        let king_bitboard: BitBoard = self.get_piece_bitboard(Piece::King, side);
        let mut n = king_bitboard.0;

        let mut i = 0;
        while n > 0 {
            let bit = n & 1;

            if bit == 1 {
                return Some(SquareLabel::from(i));
            }

            n = n >> 1;
            i += 1;
        }
        None
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

    pub fn check_en_passant(
        &mut self,
        current_from_square: SquareLabel,
        side: Side,
    ) -> Option<BitBoard> {
        if let Some(last_m) = self.last_move {
            if last_m.is_double_pawn_push() {
                let last_move_rank = get_rank(last_m.target_square());
                let last_move_file = get_file(last_m.target_square());

                let current_from_square_rank = get_rank(current_from_square);
                let current_from_square_file = get_file(current_from_square);

                /*
                println!("LAST MOVE RANK:           {:#X}", last_move_rank);
                println!("LAST MOVE FILE:           {:#X}", last_move_file);
                println!();

                println!("CURRENT FROM MOVE RANK:   {:#X}", current_from_square_rank);
                println!("CURRENT FROM MOVE FILE:   {:#X}", current_from_square_file);
                println!();
                */

                if last_move_rank == current_from_square_rank {
                    let adjacent_files = BitBoard(adjacent_files(last_m.target_square()));
                    let exist: BitBoard = (adjacent_files & BitBoard(current_from_square_file));

                    if exist != EMPTY_BITBOARD {
                        let mut en_pass_board = EMPTY_BITBOARD;
                        en_pass_board.set_bit(last_m.target_square());
                        if side == Side::White {
                            en_pass_board = en_pass_board << 8;
                        } else {
                            en_pass_board = en_pass_board >> 8;
                        }

                        let mut n = en_pass_board.0;
                        let mut i = 0;
                        while n > 0 {
                            let bit = n & 1;
                            if bit == 1 {
                                self.state.en_passant_square = Some(SquareLabel::from(i));
                                return Some(en_pass_board);
                            }

                            n = n >> 1;
                            i += 1;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn make_move(&mut self, mv: Move) {
        let from_bitboard: BitBoard = BitBoard(1) << (mv.from_square as usize);
        let to_bitboard: BitBoard = BitBoard(1) << (mv.target_square as usize);
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let enemy: Side = self.state.enemy();

        if mv.from_square != mv.target_square
            && self.side_bitboards[self.state.turn as usize].get_bit(mv.target_square as u64) == 0
        {
            if mv.piece == Piece::King {
                match self.state.turn {
                    Side::White => {
                        self.history.prev_white_king_square = Some(self.white_king_square);
                        self.white_king_square = mv.target_square;
                    }
                    Side::Black => {
                        self.history.prev_black_king_square = Some(self.black_king_square);
                        self.black_king_square = mv.target_square;
                    }
                }
            }

            self.history.prev_pieces.push(self.pieces);
            self.history.prev_main_bitboards.push(self.main_bitboard);
            self.history.prev_empty_bitboards.push(self.empty_bitboard);
            self.history.prev_piece_bitboards.push(self.piece_bitboards);
            self.history.prev_side_bitboards.push(self.side_bitboards);
            self.history.prev_states.push(self.state);

            // En passant
            if mv.move_type == MoveType::EnPassant {
                // Update piece bitboard
                self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^=
                    from_to_bitboard;

                // Update white or black bitboard
                self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                // Update main_bitboard
                self.main_bitboard ^= from_to_bitboard;

                // Update empty bitboard
                self.empty_bitboard = !self.main_bitboard;

                // Update pieces
                self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
                self.pieces[mv.from_square as usize] = None;

                let mut test_board = EMPTY_BITBOARD;
                test_board.set_bit(mv.target_square());

                if self.state.enemy() == Side::White {
                    test_board = test_board << 8;
                } else {
                    test_board = test_board >> 8;
                }

                // Clear Piece taken by en passant
                self.piece_bitboards[self.state.enemy() as usize][Piece::Pawn as usize] =
                    self.get_piece_bitboard(Piece::Pawn, self.state.enemy()) & !(test_board);

                self.main_bitboard ^= test_board;
                self.side_bitboards[self.state.enemy() as usize] ^= test_board;
                self.empty_bitboard = !self.main_bitboard;

                self.history.moves.push(mv);
                self.last_move = Some(mv);
                self.state.change_turn();
            } else {
                // Check from_square has piece on it
                if self.side_bitboards[self.state.turn as usize].get_bit(mv.from_square as u64) != 0
                {
                    if self.side_bitboards[self.state.enemy() as usize]
                        .get_bit(mv.target_square as u64)
                        == 1
                    {
                        // Update piece bitboard
                        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^=
                            from_to_bitboard;

                        // Update white or black bitboard
                        self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                        let enemy_piece =
                            self.get_piece_on_square(mv.target_square, enemy).unwrap();

                        // Reset captured piece
                        self.piece_bitboards[enemy as usize][enemy_piece as usize] ^= to_bitboard;

                        // Update color bitboard for captured side
                        self.side_bitboards[enemy as usize] ^= to_bitboard;

                        // Update main_bitboard
                        self.main_bitboard ^= from_bitboard;

                        // Update empty bitboard
                        self.empty_bitboard = !self.main_bitboard;

                        // Update pieces
                        self.pieces[mv.target_square() as usize] =
                            Some((self.state.turn, mv.piece));
                        self.pieces[mv.from_square as usize] = None;
                    } else {
                        // Update piece bitboard
                        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^=
                            from_to_bitboard;

                        // Update white or black bitboard
                        self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

                        // Update main_bitboard
                        self.main_bitboard ^= from_to_bitboard;

                        // Update empty bitboard
                        self.empty_bitboard = !self.main_bitboard;

                        // Update pieces
                        self.pieces[mv.target_square() as usize] =
                            Some((self.state.turn, mv.piece));
                        self.pieces[mv.from_square as usize] = None;
                    }

                    self.history.moves.push(mv);
                    self.last_move = Some(mv);
                    self.state.change_turn();
                }
            }
        }
    }

    pub fn unmake(&mut self) {
        // Revert BitBoards
        self.main_bitboard = self.history.prev_main_bitboards.pop().unwrap();
        self.empty_bitboard = self.history.prev_empty_bitboards.pop().unwrap();
        self.side_bitboards = self.history.prev_side_bitboards.pop().unwrap();
        self.piece_bitboards = self.history.prev_piece_bitboards.pop().unwrap();
        self.pieces = self.history.prev_pieces.pop().unwrap();

        if let Some(popped_move) = self.history.moves.pop() {
            self.last_move = Some(popped_move);
        } else {
            self.last_move = None;
        }

        self.white_king_square = self.get_king_square(Side::White).unwrap();
        self.black_king_square = self.get_king_square(Side::Black).unwrap();

        // Revert State
        self.state = self.history.prev_states.pop().unwrap();
    }

    pub fn set_bit_on_piece_bitboard(&mut self, piece: Piece, side: Side, square: SquareLabel) {
        self.piece_bitboards[side as usize][piece as usize].set_bit(square);
    }

    pub fn print_pieces(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let pos = rank * 8 + file;
                let piece = self.pieces[pos];

                if let Some(p) = piece {
                    let side = p.0;
                    let piece_type = p.1;
                    let c = piece_type.to_char(side);
                    print!("{} ", c);
                } else {
                    print!(". ");
                }
            }
            println!();
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
