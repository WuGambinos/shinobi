pub mod bitboard;
pub mod castling_rights;
pub mod generator;
pub mod mov;

use crate::{
    load_fen, mov::Move, mov::MoveType, mov::NULL_MOVE, BitBoard, MoveGenerator, Piece, Side,
    Square, Zobrist, BLACK_KINGSIDE_KING, BLACK_KINGSIDE_ROOK_FROM, BLACK_KINGSIDE_ROOK_TO,
    BLACK_QUEENSIDE_KING, BLACK_QUEENSIDE_ROOK_FROM, BLACK_QUEENSIDE_ROOK_TO, EIGTH_RANK,
    EMPTY_BITBOARD, FIRST_RANK, MAX_HALF_MOVES, START_POS, WHITE_KINGSIDE_KING,
    WHITE_KINGSIDE_ROOK_FROM, WHITE_KINGSIDE_ROOK_TO, WHITE_QUEENSIDE_KING,
    WHITE_QUEENSIDE_ROOK_FROM, WHITE_QUEENSIDE_ROOK_TO,
};

use serde::{ser::SerializeStruct, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use self::castling_rights::{Castling, CastlingRights};

pub const MAX_BOARDS: usize = 50;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct State {
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub half_move_counter: u8,
    pub full_move_counter: u8,
    pub current_turn: Side,
    pub zobrist_hash: u64,
}

impl State {
    fn new() -> State {
        State {
            castling_rights: CastlingRights::empty(),
            en_passant: None,
            half_move_counter: 0,
            full_move_counter: 1,
            current_turn: Side::White,
            zobrist_hash: 0,
        }
    }

    fn update_hash(&mut self, value: u64) {
        self.zobrist_hash ^= value;
    }

    fn change_turn(&mut self) {
        match self.current_turn {
            Side::White => self.current_turn = Side::Black,
            Side::Black => self.current_turn = Side::White,
        }
    }

    pub fn current_turn(&self) -> Side {
        self.current_turn
    }

    pub fn opponent(&self) -> Side {
        match self.current_turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    pub moves: [Move; MAX_HALF_MOVES as usize],

    /// previous Piece slice board
    pub prev_pieces: [[Option<(Side, Piece)>; 64]; MAX_BOARDS],
    pub prev_piece_count: [[[u8; 6]; 2]; MAX_BOARDS],
    pub prev_main_bitboards: [BitBoard; MAX_BOARDS],

    /// bitboards respresenting previous empty squares
    pub prev_side_bitboards: [[BitBoard; 2]; MAX_BOARDS],
    pub prev_piece_bitboards: [[[BitBoard; 6]; 2]; MAX_BOARDS],
    pub prev_states: [State; MAX_BOARDS],

    pub prev_white_king: Option<Square>,
    pub prev_black_king: Option<Square>,
    pub count: usize,
}

impl Serialize for History {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("History", 10)?;
        /*
        state.serialize_field("moves", &self.moves)?;
        state.serialize_field("prev_piece_count", &self.prev_piece_count)?;
        state.serialize_field("prev_main_bitboards", &self.prev_main_bitboards)?;
        state.serialize_field("prev_empty_bitboards", &self.prev_empty_bitboards)?;
        state.serialize_field("prev_side_bitboards", &self.prev_side_bitboards)?;
        state.serialize_field("prev_piece_bitboards", &self.prev_piece_bitboards)?;
        state.serialize_field("prev_states", &self.prev_states)?;
        state.serialize_field("prev_white_king", &self.prev_white_king)?;
        state.serialize_field("prev_black_king", &self.prev_black_king)?;
        */
        state.end()
    }
}

impl History {
    fn new() -> History {
        History {
            moves: [Move(0); MAX_HALF_MOVES as usize],
            prev_pieces: [[None; 64]; MAX_BOARDS],
            prev_piece_count: [[[0; 6]; 2]; MAX_BOARDS],
            prev_main_bitboards: [EMPTY_BITBOARD; MAX_BOARDS],
            prev_side_bitboards: [[EMPTY_BITBOARD; 2]; MAX_BOARDS],
            prev_piece_bitboards: [[[EMPTY_BITBOARD; 6]; 2]; MAX_BOARDS],
            prev_states: [State::new(); MAX_BOARDS],
            prev_white_king: None,
            prev_black_king: None,
            count: 0,
        }
    }
}

/**
 * Struct that contains all the bitboards, state, history, current king_squares,
 *
 * and last move.
 * */
#[derive(Debug, Clone)]
pub struct Position {
    /// Piece array board
    pub pieces: [Option<(Side, Piece)>; 64],
    pub main_bitboard: BitBoard,
    pub side_bitboards: [BitBoard; 2],

    /// BitBoards for all pieces and each side
    pub piece_bitboards: [[BitBoard; 6]; 2],

    pub piece_count: [[u8; 6]; 2],

    pub state: State,
    pub white_king: Square,
    pub black_king: Square,
    pub history: History,
    pub last_move: Option<Move>,
    pub zobrist: Zobrist,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Position", 11)?;
        state.serialize_field("main_bitboard", &self.main_bitboard)?;
        state.serialize_field("side_bitboards", &self.side_bitboards)?;
        state.serialize_field("piece_bitboards", &self.piece_bitboards)?;
        state.serialize_field("piece_count", &self.piece_count)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("white_king", &self.white_king)?;
        state.serialize_field("black_king", &self.black_king)?;
        state.serialize_field("history", &self.history)?;
        state.serialize_field("last_move", &self.last_move)?;
        state.serialize_field("zobrist", &self.zobrist)?;
        state.end()
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::from_fen(START_POS).unwrap()
    }
}

impl Position {
    pub fn empty() -> Position {
        Position {
            pieces: [None; 64],
            main_bitboard: EMPTY_BITBOARD,
            side_bitboards: [EMPTY_BITBOARD; 2],
            piece_bitboards: [[EMPTY_BITBOARD; 6]; 2],
            piece_count: [[0; 6]; 2],

            state: State::new(),

            white_king: Square::A1,
            black_king: Square::A1,

            history: History::new(),
            last_move: None,
            zobrist: Zobrist::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Result<Position, String> {
        let mut position = Position::empty();
        let grid = load_fen(fen, &mut position.state)?;

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
                position.white_king = Square::from(i as u64);
            } else if *ch == 'k' {
                position.black_king = Square::from(i as u64);
            }

            match ch {
                'P' | 'B' | 'N' | 'R' | 'Q' | 'K' => {
                    position.side_bitboards[Side::White as usize] |= mask;
                    position.piece_bitboards[Side::White as usize][piece] |= mask;
                    position.main_bitboard |= mask;
                    position.pieces[i] = Some((Side::White, Piece::from(*ch)));
                    position.piece_count[Side::White as usize][Piece::from(*ch) as usize] += 1;
                }
                'p' | 'b' | 'n' | 'r' | 'q' | 'k' => {
                    position.side_bitboards[Side::Black as usize] |= mask;
                    position.piece_bitboards[Side::Black as usize][piece] |= mask;
                    position.main_bitboard |= mask;
                    position.pieces[i] = Some((Side::Black, Piece::from(*ch)));
                    position.piece_count[Side::Black as usize][Piece::from(*ch) as usize] += 1;
                }
                '.' => (),

                _ => return Err("Invalid FEN".to_string()),
            }
        }

        let mut z = position.zobrist;
        position.state.zobrist_hash = z.generate_hash(&position);

        Ok(position)
    }

    pub fn opponent_bitboard(&self) -> BitBoard {
        self.side_bitboards[self.state.opponent() as usize]
    }

    pub fn set_bit_on_piece_bitboard(&mut self, piece: Piece, side: Side, square: Square) {
        self.piece_bitboards[side as usize][piece as usize].set_bit(square);
    }

    pub fn piece_bitboard(&self, piece: Piece, side: Side) -> BitBoard {
        self.piece_bitboards[side as usize][piece as usize]
    }

    pub fn king(&self, side: Side) -> Square {
        let king_bitboard: BitBoard = self.piece_bitboard(Piece::King, side);
        king_bitboard.bitscan_forward()
    }

    pub fn piece_on_square(&self, square: Square, side: Side) -> Option<Piece> {
        let pieces: [BitBoard; 6] = self.piece_bitboards[side as usize];

        for piece in Piece::iter() {
            let board = pieces[piece as usize];

            if board.get_bit(square as u64) == 1 {
                return Some(piece);
            }
        }

        None
    }

    fn castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Update king bitboard
        self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] ^=
            from_to_bitboard;

        self.queen_side_castle(mv, from_bitboard, to_bitboard);
        self.king_side_castle(mv, from_bitboard, to_bitboard);
    }

    fn queen_side_castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let mut old_rook_board: BitBoard =
            self.piece_bitboard(Piece::Rook, self.state.current_turn());

        let (castle_king, from_rook, to_rook, rank): (Square, Square, Square, BitBoard) =
            match self.state.current_turn {
                Side::White => (
                    WHITE_QUEENSIDE_KING,
                    WHITE_QUEENSIDE_ROOK_FROM,
                    WHITE_QUEENSIDE_ROOK_TO,
                    FIRST_RANK,
                ),
                Side::Black => (
                    BLACK_QUEENSIDE_KING,
                    BLACK_QUEENSIDE_ROOK_FROM,
                    BLACK_QUEENSIDE_ROOK_TO,
                    EIGTH_RANK,
                ),
            };

        let queenside_castle = mv.target() == castle_king;
        if queenside_castle {
            // Queen side squares
            let queen_side = BitBoard((1 << mv.from() as usize) - 1) & rank;
            old_rook_board &= queen_side;

            let mut new_rook_board = EMPTY_BITBOARD;
            new_rook_board.set_bit(to_rook);

            let rook_to = old_rook_board ^ new_rook_board;

            // Move rook
            self.piece_bitboards[self.state.current_turn() as usize][Piece::Rook as usize] ^=
                rook_to;

            // Update white or black bitboard
            self.side_bitboards[self.state.current_turn() as usize] ^= from_to_bitboard ^ rook_to;

            // Update main_bitboard
            self.main_bitboard ^= from_to_bitboard ^ rook_to;

            // Update piece array board
            self.pieces[mv.target() as usize] = Some((self.state.current_turn(), mv.piece()));
            self.pieces[mv.from() as usize] = None;
            self.pieces[to_rook as usize] = Some((self.state.current_turn(), Piece::Rook));
            self.pieces[from_rook as usize] = None;

            // Remove rook from hash (from_rook_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                Piece::Rook,
                from_rook,
            ));

            // Add rook to hash (to_rook_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                Piece::Rook,
                to_rook,
            ));
        }
    }

    fn king_side_castle(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let mut old_rook_board: BitBoard =
            self.piece_bitboard(Piece::Rook, self.state.current_turn());

        let (castle_king, from_rook, to_rook, rank): (Square, Square, Square, BitBoard) =
            match self.state.current_turn() {
                Side::White => (
                    WHITE_KINGSIDE_KING,
                    WHITE_KINGSIDE_ROOK_FROM,
                    WHITE_KINGSIDE_ROOK_TO,
                    FIRST_RANK,
                ),
                Side::Black => (
                    BLACK_KINGSIDE_KING,
                    BLACK_KINGSIDE_ROOK_FROM,
                    BLACK_KINGSIDE_ROOK_TO,
                    EIGTH_RANK,
                ),
            };

        // Kingside castle
        let kingside_castle = mv.target() == castle_king;
        if kingside_castle {
            // Kingside squares
            let king_side = BitBoard(!1 << mv.from() as usize) & rank;
            old_rook_board &= king_side;

            let mut new_rook_board = EMPTY_BITBOARD;
            new_rook_board.set_bit(to_rook);

            let rook_to = old_rook_board ^ new_rook_board;

            // Move rook
            self.piece_bitboards[self.state.current_turn() as usize][Piece::Rook as usize] ^=
                rook_to;

            // Update white or black bitboard
            self.side_bitboards[self.state.current_turn() as usize] ^= from_to_bitboard ^ rook_to;

            // Update main_bitboard
            self.main_bitboard ^= from_to_bitboard ^ rook_to;

            // Update piece array board
            self.pieces[mv.target() as usize] = Some((self.state.current_turn(), mv.piece()));
            self.pieces[mv.from() as usize] = None;
            self.pieces[to_rook as usize] = Some((self.state.current_turn(), Piece::Rook));
            self.pieces[from_rook as usize] = None;

            // Remove rook from hash (from_rook_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                Piece::Rook,
                from_rook,
            ));

            // Add rook to hash (to_rook_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                Piece::Rook,
                to_rook,
            ));
        }
    }

    fn update_castling_rights(&mut self, mv: Move) {
        if mv.piece().is_king() {
            match self.state.current_turn() {
                Side::White => {
                    self.history.prev_white_king = Some(self.white_king);
                    self.white_king = mv.target();

                    // Disable white caslting
                    self.state.castling_rights =
                        CastlingRights(self.state.castling_rights.0 & !Castling::WHITE_CASTLING);
                }
                Side::Black => {
                    self.history.prev_black_king = Some(self.black_king);
                    self.black_king = mv.target();

                    // Disable black castling
                    self.state.castling_rights =
                        CastlingRights(self.state.castling_rights.0 & !Castling::BLACK_CASTLING);
                }
            }
        } else if mv.piece().is_rook() {
            match self.state.current_turn() {
                Side::White => match mv.from() {
                    // Disable Kingside castling
                    WHITE_KINGSIDE_ROOK_FROM => {
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & (!Castling::WHITE_KING_SIDE),
                        )
                    }
                    // Disable Queenside castling
                    WHITE_QUEENSIDE_ROOK_FROM => {
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & (!Castling::WHITE_QUEEN_SIDE),
                        )
                    }

                    _ => {}
                },
                Side::Black => match mv.from() {
                    // Disable Kingside castling
                    BLACK_KINGSIDE_ROOK_FROM => {
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & (!Castling::BLACK_KING_SIDE),
                        )
                    }

                    // Disable Queenside castling
                    BLACK_QUEENSIDE_ROOK_FROM => {
                        self.state.castling_rights = CastlingRights(
                            self.state.castling_rights.0 & (!Castling::BLACK_QUEEN_SIDE),
                        )
                    }

                    _ => {}
                },
            }
        }
    }

    pub fn make_move(&mut self, mv: Move) {
        let from_bitboard: BitBoard = BitBoard(1) << (mv.from() as usize);
        let to_bitboard: BitBoard = BitBoard(1) << (mv.target() as usize);
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        let can_move = mv.from() != mv.target()
            && self.side_bitboards[self.state.current_turn() as usize].get_bit(mv.target() as u64)
                == 0;

        if can_move {
            self.history.prev_states[self.history.count] = self.state;
            let mut pawn_move_or_capture: bool = false;

            // Remove piece from hash (from_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                mv.piece(),
                mv.from(),
            ));

            // Add piece back hash (target_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                mv.piece(),
                mv.target(),
            ));

            //Hash castle
            self.state.update_hash(
                self.zobrist
                    .rand_castling_rights_num(self.state.castling_rights.0),
            );

            self.update_castling_rights(mv);

            // Update history
            self.history.prev_pieces[self.history.count] = self.pieces;
            self.history.prev_piece_count[self.history.count] = self.piece_count;
            self.history.prev_main_bitboards[self.history.count] = self.main_bitboard;
            self.history.prev_piece_bitboards[self.history.count] = self.piece_bitboards;
            self.history.prev_side_bitboards[self.history.count] = self.side_bitboards;

            match mv.move_type() {
                MoveType::Castle => self.castle(mv, from_bitboard, to_bitboard),
                MoveType::Promotion => {
                    self.promote(mv, from_to_bitboard, to_bitboard);
                }
                MoveType::EnPassant => self.en_passant(mv, from_bitboard, to_bitboard),
                _ => {
                    let piece_on_from_square = self.side_bitboards
                        [self.state.current_turn as usize]
                        .get_bit(mv.from() as u64)
                        != 0;
                    if piece_on_from_square {
                        let capture = self.side_bitboards[self.state.opponent() as usize]
                            .get_bit(mv.target() as u64)
                            == 1;
                        if capture {
                            pawn_move_or_capture = true;
                            self.capture(mv, from_bitboard, to_bitboard);
                        } else {
                            self.quiet(mv, from_bitboard, to_bitboard);
                        }
                    }
                }
            }

            // hash enpassant if available (remove enpassant square from hash )
            if let Some(ep) = self.state.en_passant {
                self.state.update_hash(self.zobrist.rand_en_passant(ep));
            }

            // Reset en pass square
            self.state.en_passant = None;

            // Handle Double pawn push
            if mv.is_double_pawn_push() {
                let en_passant = if self.state.current_turn() == Side::White {
                    Square::from(mv.target() as u64 - 8)
                } else {
                    Square::from(mv.target() as u64 + 8)
                };
                self.state.en_passant = Some(en_passant);
                self.state
                    .update_hash(self.zobrist.rand_en_passant(en_passant));
            }

            // Hash castle
            self.state.update_hash(
                self.zobrist
                    .rand_castling_rights_num(self.state.castling_rights.0),
            );

            pawn_move_or_capture |= mv.piece().is_pawn();

            if pawn_move_or_capture {
                self.state.half_move_counter = 0;
            } else {
                self.state.half_move_counter += 1;
            }

            if self.state.current_turn == Side::Black {
                self.state.full_move_counter += 1;
            }

            // Update history
            self.history.moves[self.history.count] = mv;
            self.last_move = Some(mv);

            self.state.update_hash(self.zobrist.rand_side_num());

            self.state.change_turn();
            self.history.count += 1;
        }
    }

    /**
     * Restores position back to state it was in before previous move was made
     */
    pub fn unmake(&mut self) {
        let index = self.history.count - 1;
        // Revert BitBoards
        self.main_bitboard = self.history.prev_main_bitboards[index];
        self.side_bitboards = self.history.prev_side_bitboards[index];
        self.piece_bitboards = self.history.prev_piece_bitboards[index];
        self.pieces = self.history.prev_pieces[index];
        self.piece_count = self.history.prev_piece_count[index];

        self.history.prev_main_bitboards[index] = EMPTY_BITBOARD;
        self.history.prev_piece_bitboards[index] = [[EMPTY_BITBOARD; 6]; 2];
        self.history.prev_side_bitboards[index] = [EMPTY_BITBOARD; 2];
        self.history.prev_pieces[index] = [None; 64];
        self.history.prev_piece_count[index] = [[0; 6]; 2];

        // Restore last move
        self.last_move = if self.history.moves[index] != NULL_MOVE {
            Some(self.history.moves[index])
        } else {
            None
        };

        self.history.moves[index] = NULL_MOVE;

        // Restore king square
        self.white_king = self.king(Side::White);
        self.black_king = self.king(Side::Black);

        // Revert state
        self.state = self.history.prev_states[index];
        self.history.count -= 1;
    }

    fn promote(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Promotion Capture
        let possible_piece: Option<(Side, Piece)> = self.pieces[mv.target() as usize];
        let mut capture: bool = false;
        if let Some(possible_p) = possible_piece {
            if possible_p.0 != self.state.current_turn() {
                let opponent: Side = self.state.opponent();
                let opponent_piece = self.piece_on_square(mv.target(), opponent).unwrap();

                // Update piece bitboard
                self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] &=
                    !from_bitboard;

                // Promote to new piece
                self.piece_bitboards[self.state.current_turn() as usize]
                    [mv.promotion_piece().unwrap() as usize] ^= to_bitboard;

                // Update side bitboard for side making the move
                self.side_bitboards[self.state.current_turn() as usize] ^= from_bitboard;

                // Reset captured piece
                self.piece_bitboards[opponent as usize][opponent_piece as usize] ^= to_bitboard;

                // Update side bitboard for captured side
                self.side_bitboards[opponent as usize] ^= to_bitboard;

                // Update main_bitboard
                self.main_bitboard ^= from_to_bitboard;

                capture = true;

                // Remove captured piece from hash (target_square)
                self.state.update_hash(self.zobrist.rand_piece_num(
                    opponent,
                    opponent_piece,
                    mv.target(),
                ));

                // Remove pawn from hash (target_square)
                self.state.update_hash(self.zobrist.rand_piece_num(
                    self.state.current_turn(),
                    Piece::Pawn,
                    mv.target(),
                ));

                // Add promoted piece to hash (target_square)
                self.state.update_hash(self.zobrist.rand_piece_num(
                    self.state.current_turn(),
                    mv.promotion_piece().unwrap(),
                    mv.target(),
                ));

                self.piece_count[opponent as usize][opponent_piece as usize] -= 1;
            }
        }

        // Quiet Promotion
        if !capture {
            // Update piece bitboard
            self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] &=
                !from_bitboard;

            // Promote to new piece
            self.piece_bitboards[self.state.current_turn() as usize]
                [mv.promotion_piece().unwrap() as usize] ^= to_bitboard;

            // Update white or black bitboard
            self.side_bitboards[self.state.current_turn() as usize] ^= from_bitboard;

            // Update main_bitboard
            self.main_bitboard ^= from_bitboard;

            // Remove pawn from hash  (target_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                Piece::Pawn,
                mv.target(),
            ));

            // Add promoted piece to hash (target_square)
            self.state.update_hash(self.zobrist.rand_piece_num(
                self.state.current_turn(),
                mv.promotion_piece().unwrap(),
                mv.target(),
            ));
        }

        // Update piece array board
        self.pieces[mv.target() as usize] =
            Some((self.state.current_turn(), mv.promotion_piece().unwrap()));
        self.pieces[mv.from() as usize] = None;
        self.piece_count[self.state.current_turn() as usize]
            [mv.promotion_piece().unwrap() as usize] += 1;
    }

    fn en_passant(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Update piece bitboard
        self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] ^=
            from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.current_turn() as usize] ^= from_to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_to_bitboard;

        // Update pieces
        self.pieces[mv.target() as usize] = Some((self.state.current_turn(), mv.piece()));
        self.pieces[mv.from() as usize] = None;

        let target_square_num: usize = if self.state.opponent() == Side::Black {
            mv.target() as usize - 8
        } else {
            mv.target() as usize + 8
        };

        self.pieces[target_square_num] = None;

        let mut ep_bitboard: BitBoard = EMPTY_BITBOARD;
        ep_bitboard.set_bit(mv.target());

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

        // Remove pawn taken from hash
        self.state.update_hash(self.zobrist.rand_piece_num(
            self.state.opponent(),
            Piece::Pawn,
            Square::from(target_square_num as u64),
        ));
    }

    pub fn capture(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;
        let opponent: Side = self.state.opponent();

        // Update piece bitboard
        self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] ^=
            from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.current_turn() as usize] ^= from_to_bitboard;

        let opponent_piece: Piece = self.piece_on_square(mv.target(), opponent).unwrap();

        // Hash capture
        self.state.update_hash(
            self.zobrist
                .rand_piece_num(opponent, opponent_piece, mv.target()),
        );

        // Reset captured piece
        self.piece_bitboards[opponent as usize][opponent_piece as usize] ^= to_bitboard;

        // Update color bitboard for captured side
        self.side_bitboards[opponent as usize] ^= to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_bitboard;

        // Update piece array board
        self.pieces[mv.target() as usize] = Some((self.state.current_turn(), mv.piece()));
        self.pieces[mv.from() as usize] = None;

        self.piece_count[opponent as usize][opponent_piece as usize] -= 1;
    }
    fn quiet(&mut self, mv: Move, from_bitboard: BitBoard, to_bitboard: BitBoard) {
        let from_to_bitboard: BitBoard = from_bitboard ^ to_bitboard;

        // Update piece bitboard
        self.piece_bitboards[self.state.current_turn() as usize][mv.piece() as usize] ^=
            from_to_bitboard;

        // Update white or black bitboard
        self.side_bitboards[self.state.current_turn() as usize] ^= from_to_bitboard;

        // Update main_bitboard
        self.main_bitboard ^= from_to_bitboard;

        // Update piece array board
        self.pieces[mv.target() as usize] = Some((self.state.current_turn(), mv.piece()));
        self.pieces[mv.from() as usize] = None;
    }

    pub fn checkmate(&mut self, move_gen: &MoveGenerator) -> bool {
        let side = self.state.current_turn();
        move_gen.generate_legal_moves(self, side).is_empty()
            && move_gen.attacks_to_king(self, side) != EMPTY_BITBOARD
    }

    pub fn is_draw(&mut self, move_gen: &MoveGenerator) -> bool {
        self.draw_by_fifty_moves()
            | self.draw_by_threefold_repetition()
            | self.draw_by_insufficient_material()
            | self.stalemate(move_gen)
    }

    fn stalemate(&mut self, move_gen: &MoveGenerator) -> bool {
        let side = self.state.current_turn();
        move_gen.generate_legal_moves(self, side).is_empty()
            && move_gen.attacks_to_king(self, side) != EMPTY_BITBOARD
    }

    fn draw_by_fifty_moves(&self) -> bool {
        self.state.half_move_counter >= MAX_HALF_MOVES
    }

    fn draw_by_threefold_repetition(&mut self) -> bool {
        let current_pos_hash = self.state.zobrist_hash;
        let prev_states = &self.history.prev_states;

        let mut count = 1;
        for state in prev_states {
            if state.zobrist_hash == current_pos_hash {
                count += 1;
            }

            if count == 3 {
                return true;
            }
        }

        false
    }

    /**
     * TODO
     *
     * */
    pub fn draw_by_insufficient_material(&self) -> bool {
        false
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

    pub fn print_piece_count(&self) {
        let mut white_total = 0;
        for piece in Piece::iter() {
            white_total += self.piece_count[Side::White as usize][piece as usize];
            print!(
                "Piece: {:?}: COUNT: {} ",
                piece,
                self.piece_count[Side::White as usize][piece as usize]
            );
        }
        println!("WHITE TOTAL: {}", white_total);
        println!();

        let mut black_total = 0;
        for piece in Piece::iter() {
            black_total += self.piece_count[Side::Black as usize][piece as usize];
            print!(
                "Piece: {:?}: COUNT: {} ",
                piece,
                self.piece_count[Side::Black as usize][piece as usize]
            );
        }
        println!("BLACK TOTAL: {}", black_total);
        println!();
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let pos = rank * 8 + file;
                let piece = self.pieces[pos];

                if let Some(p) = piece {
                    let side = p.0;
                    let piece_type = p.1;
                    let c: String = piece_type.to_char(side).to_string();
                    write!(f, "{} ", c)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Side To Move: {:?}", self.state.current_turn)?;
        writeln!(f,)?;
        writeln!(f, "Half Move Counter: {}", self.state.half_move_counter)?;
        writeln!(f,)?;
        writeln!(f, "Full Move Counter: {}", self.state.full_move_counter)?;
        writeln!(f,)?;
        writeln!(f, "Castling Rights: {:?}", self.state.castling_rights)?;
        writeln!(f,)?;
        writeln!(f, "En Passant Square: {:?}", self.state.en_passant)?;
        writeln!(f,)?;
        writeln!(f, "HASH: {:#X}", self.state.zobrist_hash)
    }
}
