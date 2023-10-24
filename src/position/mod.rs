pub mod bitboard;
pub mod castling_rights;
pub mod generator;
use crate::{
    adjacent_files, get_file, get_rank, load_fen, square_name, BitBoard, MoveGenerator, Piece,
    Side, SquareLabel, Zobrist, BLACK_KINGSIDE_KING_SQUARE, BLACK_KINGSIDE_ROOK_FROM_SQUARE,
    BLACK_KINGSIDE_ROOK_TO_SQUARE, BLACK_QUEENSIDE_KING_SQUARE, BLACK_QUEENSIDE_ROOK_FROM_SQUARE,
    BLACK_QUEENSIDE_ROOK_TO_SQUARE, EIGTH_RANK, EMPTY_BITBOARD, FIRST_RANK, MAX_HALF_MOVES,
    WHITE_KINGSIDE_KING_SQUARE, WHITE_KINGSIDE_ROOK_FROM_SQUARE, WHITE_KINGSIDE_ROOK_TO_SQUARE,
    WHITE_QUEENSIDE_KING_SQUARE, WHITE_QUEENSIDE_ROOK_FROM_SQUARE, WHITE_QUEENSIDE_ROOK_TO_SQUARE,
};

use std::fmt;
use strum::IntoEnumIterator;

use self::castling_rights::{Castling, CastlingRights};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<SquareLabel>,
    pub half_move_counter: u8,
    pub full_move_counter: u8,
    pub turn: Side,
    pub zobrist_key: u64,
}

impl State {
    /// Create state struct
    fn new() -> State {
        State {
            castling_rights: CastlingRights::empty(),
            en_passant_square: None,
            half_move_counter: 0,
            full_move_counter: 1,
            turn: Side::White,
            zobrist_key: 0,
        }
    }

    /// Switch turn from white to black or vice versa
    fn change_turn(&mut self) {
        match self.turn {
            Side::White => self.turn = Side::Black,
            Side::Black => self.turn = Side::White,
        }
    }

    /// Return current opponent
    pub fn opponent(&self) -> Side {
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
    Castle,
    Promotion,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    /// Piece Moving
    pub piece: Piece,

    pub from_square: SquareLabel,
    pub target_square: SquareLabel,
    pub move_type: MoveType,
    pub promotion_piece: Option<Piece>,
}

impl Move {
    /// Creates move struct
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
            promotion_piece: None,
        }
    }

    pub fn with_promotion_piece(
        piece: Piece,
        from_square: SquareLabel,
        target_square: SquareLabel,
        move_type: MoveType,
        promotion_piece: Option<Piece>,
    ) -> Move {
        Move {
            piece,
            from_square,
            target_square,
            move_type,
            promotion_piece,
        }
    }

    /// Returns if move was a double pawn push
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

    fn move_type(&self) -> MoveType {
        self.move_type
    }
}

/// Prints Move in format like "a1b2"
///
/// Where a1 is the from square and b2 is the target_square
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.move_type == MoveType::Promotion {
            // promotion move format like "c7c8q" (pawn from c7 to c8  and promoting to queen)
            write!(
                f,
                "{}{}{}",
                square_name(self.from_square() as u8),
                square_name(self.target_square() as u8),
                self.promotion_piece.unwrap().to_char(Side::Black)
            )
        } else {
            write!(
                f,
                "{}{}",
                square_name(self.from_square() as u8),
                square_name(self.target_square() as u8)
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    /// Vec of previous moves
    pub moves: Vec<Move>,

    /// Vec of previous Piece slice board
    pub prev_pieces: Vec<[Option<(Side, Piece)>; 64]>,

    /// Vec of bitboards representing main board
    pub prev_main_bitboards: Vec<BitBoard>,

    /// Vec of bitboards respresenting previous empty squares
    pub prev_empty_bitboards: Vec<BitBoard>,

    // Vec of bitboards representing previous side bitboards
    pub prev_side_bitboards: Vec<[BitBoard; 2]>,

    // Vec of bitboards representing previous piece bitboards
    pub prev_piece_bitboards: Vec<[[BitBoard; 6]; 2]>,

    /// Vec of previos position states
    pub prev_states: Vec<State>,

    pub prev_white_king_square: Option<SquareLabel>,
    pub prev_black_king_square: Option<SquareLabel>,
}

impl History {
    /// Create history struct
    fn new() -> History {
        History {
            moves: Vec::with_capacity(50),
            prev_pieces: Vec::with_capacity(50),
            prev_main_bitboards: Vec::with_capacity(50),
            prev_empty_bitboards: Vec::with_capacity(50),
            prev_side_bitboards: Vec::with_capacity(50),
            prev_piece_bitboards: Vec::with_capacity(50),
            prev_states: Vec::with_capacity(50),
            prev_white_king_square: None,
            prev_black_king_square: None,
        }
    }
}

/// Struct that contains all the bitboards, move generator, state, history, current king_squares,
///
/// and last move.
#[derive(Debug, Clone)]
pub struct Position {
    /// Piece array board
    pub pieces: [Option<(Side, Piece)>; 64],

    /// BitBoard that shows combined states of white and black bitboards
    pub main_bitboard: BitBoard,

    /// BitBoard showing which squares are empty
    pub empty_bitboard: BitBoard,

    /// BitBoard for each side
    pub side_bitboards: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub piece_bitboards: [[BitBoard; 6]; 2],

    /// State contains all relveant information for evalution
    pub state: State,

    pub white_king_square: SquareLabel,
    pub black_king_square: SquareLabel,

    pub history: History,
    pub last_move: Option<Move>,

    pub zobrist: Zobrist,
}

impl Position {
    /// Create position struct
    pub fn empty() -> Position {
        Position {
            pieces: [None; 64],
            main_bitboard: EMPTY_BITBOARD,
            empty_bitboard: EMPTY_BITBOARD,
            side_bitboards: [EMPTY_BITBOARD; 2],
            piece_bitboards: [[EMPTY_BITBOARD; 6]; 2],

            state: State::new(),

            white_king_square: SquareLabel::A1,
            black_king_square: SquareLabel::A1,

            history: History::new(),
            last_move: None,
            zobrist: Zobrist::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Position {
        let mut position = Position {
            pieces: [None; 64],
            main_bitboard: EMPTY_BITBOARD,
            empty_bitboard: EMPTY_BITBOARD,
            side_bitboards: [EMPTY_BITBOARD; 2],
            piece_bitboards: [[EMPTY_BITBOARD; 6]; 2],

            state: State::new(),

            white_king_square: SquareLabel::A1,
            black_king_square: SquareLabel::A1,

            history: History::new(),
            last_move: None,
            zobrist: Zobrist::new(),
        };

        let grid = load_fen(&fen, &mut position.state);

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
                position.white_king_square = SquareLabel::from(i as u64);
            } else if *ch == 'k' {
                position.black_king_square = SquareLabel::from(i as u64);
            }

            if ch.is_ascii() {
                if ch.is_uppercase() {
                    position.side_bitboards[Side::White as usize] |= mask;
                    position.piece_bitboards[Side::White as usize][piece] |= mask;
                    position.main_bitboard |= mask;
                    position.pieces[i] = Some((Side::White, Piece::from(*ch)));
                } else if ch.is_lowercase() {
                    position.side_bitboards[Side::Black as usize] |= mask;
                    position.piece_bitboards[Side::Black as usize][piece] |= mask;
                    position.main_bitboard |= mask;
                    position.pieces[i] = Some((Side::Black, Piece::from(*ch)));
                } else {
                    position.empty_bitboard |= mask;
                }
            }
        }

        let mut z = position.zobrist;
        position.state.zobrist_key = z.generate_hash_key(&position);

        position
    }

    pub fn opponent_bitboard(&self) -> BitBoard {
        return self.side_bitboards[self.state.opponent() as usize];
    }

    pub fn set_bit_on_piece_bitboard(&mut self, piece: Piece, side: Side, square: SquareLabel) {
        self.piece_bitboards[side as usize][piece as usize].set_bit(square);
    }

    /// Returns bitboard for a specific side and piece
    pub fn piece_bitboard(&self, piece: Piece, side: Side) -> BitBoard {
        self.piece_bitboards[side as usize][piece as usize]
    }

    pub fn king_square(&self, side: Side) -> SquareLabel {
        let king_bitboard: BitBoard = self.piece_bitboard(Piece::King, side);
        return king_bitboard.bitscan_forward();
    }

    // Return Some piece on spefic square is there is one , None otherwise
    pub fn piece_on_square(&self, square: SquareLabel, side: Side) -> Option<Piece> {
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
            // Last move was double pawn push
            if last_m.is_double_pawn_push() {
                let last_move_rank = get_rank(last_m.target_square());
                let last_move_file = get_file(last_m.target_square());

                let current_from_square_rank = get_rank(current_from_square);
                let current_from_square_file = get_file(current_from_square);

                if last_move_rank == current_from_square_rank {
                    let adjacent_files = adjacent_files(last_m.target_square());
                    let exist: BitBoard = adjacent_files & current_from_square_file;

                    // Check if there is a piece on adjacent file
                    if exist != EMPTY_BITBOARD {
                        let mut ep_board = EMPTY_BITBOARD;
                        ep_board.set_bit(last_m.target_square());
                        if side == Side::White {
                            ep_board = ep_board << 8;
                        } else {
                            ep_board = ep_board >> 8;
                        }

                        self.state.en_passant_square = Some(ep_board.bitscan_forward());
                        return Some(ep_board);
                    }
                }
            }
        }
        None
    }

    pub fn queen_side_castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard = from_bitboard ^ to_bitboard;
        let mut old_rook_board = self.piece_bitboard(Piece::Rook, self.state.turn);

        let (castle_king_square, from_rook_square, to_rook_square, rank) = match self.state.turn {
            Side::White => (
                WHITE_QUEENSIDE_KING_SQUARE,
                WHITE_QUEENSIDE_ROOK_FROM_SQUARE,
                WHITE_QUEENSIDE_ROOK_TO_SQUARE,
                FIRST_RANK,
            ),
            Side::Black => (
                BLACK_QUEENSIDE_KING_SQUARE,
                BLACK_QUEENSIDE_ROOK_FROM_SQUARE,
                BLACK_QUEENSIDE_ROOK_TO_SQUARE,
                EIGTH_RANK,
            ),
        };

        // Queenside castle
        if mv.target_square() == castle_king_square {
            // Queen side squares
            let queen_side = BitBoard((1 << mv.from_square() as usize) - 1) & rank;
            old_rook_board = old_rook_board & queen_side;

            let mut new_rook_board = EMPTY_BITBOARD;
            new_rook_board.set_bit(to_rook_square);

            let rook_to = old_rook_board ^ new_rook_board;

            // Move rook
            self.piece_bitboards[self.state.turn as usize][Piece::Rook as usize] ^= rook_to;

            // Update white or black bitboard
            self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard ^ rook_to;

            // Update main_bitboard
            self.main_bitboard ^= from_to_bitboard ^ rook_to;

            // Update empty bitboard
            self.empty_bitboard = !self.main_bitboard;

            // Update piece array board
            self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
            self.pieces[mv.from_square as usize] = None;
            self.pieces[to_rook_square as usize] = Some((self.state.turn, Piece::Rook));
            self.pieces[from_rook_square as usize] = None;
        }
    }

    pub fn king_side_castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard = from_bitboard ^ to_bitboard;
        let mut old_rook_board = self.piece_bitboard(Piece::Rook, self.state.turn);

        let (castle_king_square, from_rook_square, to_rook_square, rank) = match self.state.turn {
            Side::White => (
                WHITE_KINGSIDE_KING_SQUARE,
                WHITE_KINGSIDE_ROOK_FROM_SQUARE,
                WHITE_KINGSIDE_ROOK_TO_SQUARE,
                FIRST_RANK,
            ),
            Side::Black => (
                BLACK_KINGSIDE_KING_SQUARE,
                BLACK_KINGSIDE_ROOK_FROM_SQUARE,
                BLACK_KINGSIDE_ROOK_TO_SQUARE,
                EIGTH_RANK,
            ),
        };

        // Kingside castle
        if mv.target_square() == castle_king_square {
            // Kingside squares
            let king_side = BitBoard(!1 << mv.from_square() as usize) & rank;
            old_rook_board = old_rook_board & king_side;

            let mut new_rook_board = EMPTY_BITBOARD;
            new_rook_board.set_bit(to_rook_square);

            let rook_to = old_rook_board ^ new_rook_board;

            // Move rook
            self.piece_bitboards[self.state.turn as usize][Piece::Rook as usize] ^= rook_to;

            // Update white or black bitboard
            self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard ^ rook_to;

            // Update main_bitboard
            self.main_bitboard ^= from_to_bitboard ^ rook_to;

            // Update empty bitboard
            self.empty_bitboard = !self.main_bitboard;

            // Update piece array board
            self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
            self.pieces[mv.from_square as usize] = None;
            self.pieces[to_rook_square as usize] = Some((self.state.turn, Piece::Rook));
            self.pieces[from_rook_square as usize] = None;
        }
    }

    pub fn castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard = from_bitboard ^ to_bitboard;

        // Update king bitboard
        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^= from_to_bitboard;

        // Castle
        self.queen_side_castle(mv, from_bitboard, to_bitboard);
        self.king_side_castle(mv, from_bitboard, to_bitboard);
    }

    pub fn promote(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard = from_bitboard ^ to_bitboard;

        // Promotion Capture
        let possible_piece = self.pieces[mv.target_square() as usize];
        let mut capture = false;
        if let Some(possible_p) = possible_piece {
            if possible_p.0 != self.state.turn {
                let opponent: Side = self.state.opponent();
                let enemy_piece = self.piece_on_square(mv.target_square, opponent).unwrap();

                // Update piece bitboard
                self.piece_bitboards[self.state.turn as usize][mv.piece as usize] &= !from_bitboard;

                // Promote to new piece
                self.piece_bitboards[self.state.turn as usize]
                    [mv.promotion_piece.unwrap() as usize] ^= to_bitboard;

                // Update side bitboard for side making the move
                self.side_bitboards[self.state.turn as usize] ^= from_bitboard;

                // Reset captured piece
                self.piece_bitboards[opponent as usize][enemy_piece as usize] ^= to_bitboard;

                // Update side bitboard for captured side
                self.side_bitboards[opponent as usize] ^= to_bitboard;

                // Update main_bitboard
                self.main_bitboard ^= from_to_bitboard;

                // Update empty bitboard
                self.empty_bitboard = !self.main_bitboard;

                capture = true;
            }
        }

        // Quiet Promotion
        if !capture {
            // Update piece bitboard
            self.piece_bitboards[self.state.turn as usize][mv.piece as usize] &= !from_bitboard;

            // Promote to new piece
            self.piece_bitboards[self.state.turn as usize][mv.promotion_piece.unwrap() as usize] ^=
                to_bitboard;

            // Update white or black bitboard
            self.side_bitboards[self.state.turn as usize] ^= from_bitboard;

            // Update main_bitboard
            self.main_bitboard ^= from_bitboard;

            // Update empty bitboard
            self.empty_bitboard = !self.main_bitboard;
        }
        // Update piece array board
        self.pieces[mv.target_square() as usize] =
            Some((self.state.turn, mv.promotion_piece.unwrap()));
        self.pieces[mv.from_square as usize] = None;
    }

    pub fn en_passant(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard = from_bitboard ^ to_bitboard;
        // Update piece bitboard
        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^= from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_to_bitboard;

        // Update empty bitboard
        self.empty_bitboard = !self.main_bitboard;

        // Update pieces
        self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
        self.pieces[mv.from_square as usize] = None;

        if self.state.opponent() == Side::Black {
            self.pieces[mv.target_square() as usize - 8] = None;
        } else {
            self.pieces[mv.target_square() as usize + 8] = None;
        }

        let mut ep_bitboard = EMPTY_BITBOARD;
        ep_bitboard.set_bit(mv.target_square());

        if self.state.opponent() == Side::White {
            ep_bitboard = ep_bitboard << 8;
        } else {
            ep_bitboard = ep_bitboard >> 8;
        }

        // Clear Piece taken by en passant
        self.piece_bitboards[self.state.opponent() as usize][Piece::Pawn as usize] =
            self.piece_bitboard(Piece::Pawn, self.state.opponent()) & !(ep_bitboard);

        // Update mainboard
        self.main_bitboard ^= ep_bitboard;

        // Update side bitboard
        self.side_bitboards[self.state.opponent() as usize] ^= ep_bitboard;

        // Update empty bitboard
        self.empty_bitboard = !self.main_bitboard;
    }

    pub fn capture(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let opponent: Side = self.state.opponent();

        // Update piece bitboard
        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^= from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

        let enemy_piece = self.piece_on_square(mv.target_square, opponent).unwrap();

        // Reset captured piece
        self.piece_bitboards[opponent as usize][enemy_piece as usize] ^= to_bitboard;

        // Update color bitboard for captured side
        self.side_bitboards[opponent as usize] ^= to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_bitboard;

        // Update empty bitboard
        self.empty_bitboard = !self.main_bitboard;

        // Update piece array board
        self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
        self.pieces[mv.from_square as usize] = None;
    }
    pub fn quiet(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Update piece bitboard
        self.piece_bitboards[self.state.turn as usize][mv.piece as usize] ^= from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.turn as usize] ^= from_to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_to_bitboard;

        // Update empty bitboard
        self.empty_bitboard = !self.main_bitboard;

        // Update piece array board
        self.pieces[mv.target_square() as usize] = Some((self.state.turn, mv.piece));
        self.pieces[mv.from_square as usize] = None;
    }

    /// Makes move on bitboards if valid
    pub fn make_move(&mut self, mv: Move) {
        let from_bitboard: BitBoard = BitBoard(1) << (mv.from_square as usize);
        let to_bitboard: BitBoard = BitBoard(1) << (mv.target_square as usize);
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        if mv.from_square != mv.target_square
            && self.side_bitboards[self.state.turn as usize].get_bit(mv.target_square as u64) == 0
        {
            self.history.prev_states.push(self.state);
            let mut pawn_move_or_capture = false;
            // King move
            if mv.piece == Piece::King {
                match self.state.turn {
                    Side::White => {
                        self.history.prev_white_king_square = Some(self.white_king_square);
                        self.white_king_square = mv.target_square;

                        // Disable white caslting
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & !Castling::WHITE_CASTLING,
                        );
                    }
                    Side::Black => {
                        self.history.prev_black_king_square = Some(self.black_king_square);
                        self.black_king_square = mv.target_square;

                        //Disable black castling
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & !Castling::BLACK_CASTLING,
                        );
                    }
                }
            }

            if mv.piece == Piece::Rook {
                match self.state.turn {
                    Side::White => match mv.from_square() {
                        // Disable Kingside castling
                        WHITE_KINGSIDE_ROOK_FROM_SQUARE => {
                            self.state.castling_rights = CastlingRights(
                                self.state.castling_rights.0 & (!Castling::WHITE_KING_SIDE),
                            )
                        }
                        // Disable Queenside castling
                        WHITE_QUEENSIDE_ROOK_FROM_SQUARE => {
                            self.state.castling_rights = CastlingRights(
                                self.state.castling_rights.0 & (!Castling::WHITE_QUEEN_SIDE),
                            )
                        }

                        _ => {}
                    },
                    Side::Black => match mv.from_square() {
                        // Disable Kingside castling
                        BLACK_KINGSIDE_ROOK_FROM_SQUARE => {
                            self.state.castling_rights = CastlingRights(
                                self.state.castling_rights.0 & (!Castling::BLACK_KING_SIDE),
                            )
                        }

                        // Disable Queenside castling
                        BLACK_QUEENSIDE_ROOK_FROM_SQUARE => {
                            self.state.castling_rights = CastlingRights(
                                self.state.castling_rights.0 & (!Castling::BLACK_QUEEN_SIDE),
                            )
                        }

                        _ => {}
                    },
                }
            }

            // Update history
            self.history.prev_pieces.push(self.pieces);
            self.history.prev_main_bitboards.push(self.main_bitboard);
            self.history.prev_empty_bitboards.push(self.empty_bitboard);
            self.history.prev_piece_bitboards.push(self.piece_bitboards);
            self.history.prev_side_bitboards.push(self.side_bitboards);

            match mv.move_type() {
                MoveType::Castle => self.castle(mv, from_bitboard, to_bitboard),
                MoveType::Promotion => self.promote(mv, from_to_bitboard, to_bitboard),
                MoveType::EnPassant => self.en_passant(mv, from_bitboard, to_bitboard),
                _ => {
                    let piece_on_from_square = self.side_bitboards[self.state.turn as usize]
                        .get_bit(mv.from_square as u64)
                        != 0;
                    if piece_on_from_square {
                        let capture = self.side_bitboards[self.state.opponent() as usize]
                            .get_bit(mv.target_square as u64)
                            == 1;
                        if capture {
                            pawn_move_or_capture = true;
                            self.capture(mv, from_bitboard, to_bitboard);
                        }
                        // Quiet
                        else {
                            self.quiet(mv, from_bitboard, to_bitboard);
                        }
                    }
                }
            }

            if mv.piece.is_pawn() {
                pawn_move_or_capture = true;
            }

            if pawn_move_or_capture {
                self.state.half_move_counter = 0;
            } else {
                self.state.half_move_counter += 1;
            }

            if self.state.turn == Side::Black {
                self.state.full_move_counter += 1;
            }

            // Update history
            self.history.moves.push(mv);

            // Update hash
            /*
            self.state.zobrist_key ^= self.zobrist.rand_piece_num(mv.piece, mv.from_square());
            if mv.move_type == MoveType::Capture {
                self.state.zobrist_key ^= self.zobrist.rand_piece_num(
                    self.piece_on_square(mv.target_square(), self.state.opponent())
                        .unwrap(),
                    mv.target_square(),
                );
            }
            self.state.zobrist_key ^= self.zobrist.rand_piece_num(mv.piece, mv.target_square());

            if self.state.turn == Side::Black {
                self.state.zobrist_key ^= self.zobrist.rand_side_num();
            }
            */

            self.last_move = Some(mv);
            self.state.change_turn();
        }
    }

    /// Undoes the previous move made
    pub fn unmake(&mut self) {
        // Revert BitBoards
        self.main_bitboard = self.history.prev_main_bitboards.pop().unwrap();
        self.empty_bitboard = self.history.prev_empty_bitboards.pop().unwrap();
        self.side_bitboards = self.history.prev_side_bitboards.pop().unwrap();
        self.piece_bitboards = self.history.prev_piece_bitboards.pop().unwrap();
        self.pieces = self.history.prev_pieces.pop().unwrap();

        // Restore last move
        if let Some(popped_move) = self.history.moves.pop() {
            self.last_move = Some(popped_move);
        } else {
            self.last_move = None;
        }

        // Restore king square
        self.white_king_square = self.king_square(Side::White);
        self.black_king_square = self.king_square(Side::Black);

        // Revert State
        self.state = self.history.prev_states.pop().unwrap();
    }

    /// Print piece array board
    pub fn print_position(&self) {
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

        println!();
        println!("Side To Move: {:?}", self.state.turn);
        println!();
        println!("Half Move Counter: {}", self.state.half_move_counter);
        println!();
        println!("Full Move Counter: {}", self.state.full_move_counter);
        println!();
        println!("Castling Rights: {:?}", self.state.castling_rights);
        println!();
        println!("En Passant Square: {:?}", self.state.en_passant_square);
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

    pub fn print_black_bitboard(&self) {
        self.side_bitboards[Side::Black as usize].print();
    }

    pub fn print_white_bitboard(&self) {
        self.side_bitboards[Side::White as usize].print();
    }
}
