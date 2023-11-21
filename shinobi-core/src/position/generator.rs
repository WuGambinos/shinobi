use crate::{
    init_slider_attacks, mov::Move, mov::MoveType, BitBoard, Piece, Position, SMagic, Side, Square,
    A_FILE, BLACK_KINGSIDE_KING, BLACK_QUEENSIDE_KING, B_FILE, EIGTH_RANK, EMPTY_BITBOARD,
    FIFTH_RANK, FIRST_RANK, FOURTH_RANK, G_FILE, H_FILE, NUM_SIDES, NUM_SQUARES,
    WHITE_KINGSIDE_KING, WHITE_QUEENSIDE_KING,
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
pub struct MoveGenerator {
    pub knight_moves: [BitBoard; NUM_SQUARES],
    pub pawn_pushes: [[BitBoard; NUM_SQUARES]; NUM_SIDES],
    pub pawn_attacks: [[BitBoard; NUM_SQUARES]; NUM_SIDES],
    pub king_moves: [BitBoard; NUM_SQUARES],
    pub rook_moves: [BitBoard; 102400],
    pub bishop_moves: [BitBoard; 5248],
    pub bishop_tbl: [SMagic; NUM_SQUARES],
    pub rook_tbl: [SMagic; NUM_SQUARES],
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let mut move_gen = MoveGenerator {
            rook_moves: [EMPTY_BITBOARD; 102400],
            bishop_moves: [EMPTY_BITBOARD; 5248],
            knight_moves: [EMPTY_BITBOARD; NUM_SQUARES],
            pawn_pushes: [[EMPTY_BITBOARD; NUM_SQUARES]; NUM_SIDES],
            pawn_attacks: [[EMPTY_BITBOARD; NUM_SQUARES]; NUM_SIDES],
            king_moves: [EMPTY_BITBOARD; NUM_SQUARES],
            bishop_tbl: [SMagic::new(0, 0, 0, 0); NUM_SQUARES],
            rook_tbl: [SMagic::new(0, 0, 0, 0); NUM_SQUARES],
        };

        move_gen.fill_knight_moves();
        move_gen.fill_king_moves();
        move_gen.fill_pawn_attacks(Side::White);
        move_gen.fill_pawn_attacks(Side::Black);
        init_slider_attacks(&mut move_gen, true);
        init_slider_attacks(&mut move_gen, false);

        move_gen
    }

    fn north_one(&self, bitboard: BitBoard) -> BitBoard {
        bitboard << 8
    }

    fn south_one(&self, bitboard: BitBoard) -> BitBoard {
        bitboard >> 8
    }

    fn east_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 1) & !A_FILE
    }

    fn west_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 1) & !H_FILE
    }

    fn north_east_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 9) & !A_FILE
    }

    fn north_west_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 7) & !H_FILE
    }

    fn south_east_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 7) & !A_FILE
    }

    fn south_west_one(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 9) & !H_FILE
    }

    fn north_north_east(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 17) & !A_FILE
    }

    fn north_east_east(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 10) & !(A_FILE | B_FILE)
    }

    fn south_east_east(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 6) & !(A_FILE | B_FILE)
    }

    fn south_south_east(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 15) & !A_FILE
    }

    fn north_north_west(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 15) & !H_FILE
    }

    fn north_west_west(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard << 6) & !(G_FILE | H_FILE)
    }

    fn south_west_west(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 10) & !(G_FILE | H_FILE)
    }

    fn south_south_west(&self, bitboard: BitBoard) -> BitBoard {
        (bitboard >> 17) & !H_FILE
    }

    fn gen_pawn_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let turn = position.state.current_turn();
        let mut pawn: BitBoard = position.piece_bitboard(Piece::Pawn, turn);

        while pawn != EMPTY_BITBOARD {
            let mut pushes = EMPTY_BITBOARD;
            let mut bb_pawns: BitBoard = EMPTY_BITBOARD;
            let from = pawn.bitscan_forward_reset();
            bb_pawns.set_bit(from);

            if turn == Side::White {
                let single_push = self.white_single_push_target(position, bb_pawns);
                let double_push = self.white_double_push_target(position, bb_pawns);
                pushes |= single_push;
                if single_push != EMPTY_BITBOARD {
                    pushes |= double_push;
                }
            } else {
                pushes |= self.black_single_push_target(position, bb_pawns);
                pushes |= self.black_double_push_target(position, bb_pawns);
            }

            self.create_moves(position, Piece::Pawn, turn, pushes, from, &mut moves);
        }

        return moves;
    }

    fn white_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        let single_pushes = self.white_single_push_target(position, bitboard);

        self.north_one(single_pushes) & position.empty_bitboard & FOURTH_RANK
    }

    fn white_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        self.north_one(bitboard) & position.empty_bitboard
    }

    fn black_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        let single_pushes = self.black_single_push_target(position, bitboard);

        self.south_one(single_pushes) & position.empty_bitboard & FIFTH_RANK
    }

    fn black_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        self.south_one(bitboard) & position.empty_bitboard
    }

    fn fill_pawn_attacks(&mut self, side: Side) {
        for square in Square::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            match side {
                Side::White => {
                    moves |= self.white_pawn_east_attacks(bitboard);
                    moves |= self.white_pawn_west_attacks(bitboard);
                    self.pawn_attacks[side as usize][square as usize] = moves;
                }
                Side::Black => {
                    moves |= self.black_pawn_east_attacks(bitboard);
                    moves |= self.black_pawn_west_attacks(bitboard);
                    self.pawn_attacks[side as usize][square as usize] = moves;
                }
            }
        }
    }

    fn white_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        self.north_east_one(bitboard)
    }

    fn white_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        self.north_west_one(bitboard)
    }

    fn black_pawn_east_attacks(&self, bitboard: BitBoard) -> BitBoard {
        self.south_east_one(bitboard)
    }

    fn black_pawn_west_attacks(&self, bitboard: BitBoard) -> BitBoard {
        self.south_west_one(bitboard)
    }

    fn castle_squares_attacked(
        &self,
        position: &Position,
        side: Side,
        castle_squares: BitBoard,
    ) -> bool {
        let mut bb: BitBoard = castle_squares;
        let opponent: Side = position.state.opponent();

        let mut result_board: BitBoard = EMPTY_BITBOARD;
        let opponent_pawns: BitBoard = position.piece_bitboard(Piece::Pawn, opponent);
        let opponent_knights: BitBoard = position.piece_bitboard(Piece::Knight, opponent);
        let opponent_rooks: BitBoard = position.piece_bitboard(Piece::Rook, opponent);
        let opponent_bishop: BitBoard = position.piece_bitboard(Piece::Bishop, opponent);
        let opponent_queen: BitBoard = position.piece_bitboard(Piece::Queen, opponent);

        while bb != EMPTY_BITBOARD {
            let square = bb.bitscan_forward_reset();
            result_board |= (self.get_bishop_moves(square as u64, position.main_bitboard)
                & opponent_bishop)
                | (self.get_rook_moves(square as u64, position.main_bitboard) & opponent_rooks)
                | (self.knight_moves[square as usize] & opponent_knights)
                | (self.pawn_attacks[side as usize][square as usize] & opponent_pawns)
                | (self.get_queen_moves(square as u64, position.main_bitboard) & opponent_queen);
        }

        result_board != EMPTY_BITBOARD
    }

    fn fill_king_moves(&mut self) {
        for square in Square::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            moves |= self.east_one(bitboard) | self.west_one(bitboard);
            bitboard |= moves;
            moves |= self.north_one(bitboard) | self.south_one(bitboard);

            self.king_moves[square as usize] = moves;
        }
    }

    fn fill_knight_moves(&mut self) {
        for square in Square::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            moves |= self.north_north_east(bitboard);
            moves |= self.north_east_east(bitboard);
            moves |= self.south_east_east(bitboard);
            moves |= self.south_south_east(bitboard);
            moves |= self.north_north_west(bitboard);
            moves |= self.north_west_west(bitboard);
            moves |= self.south_west_west(bitboard);
            moves |= self.south_south_west(bitboard);

            self.knight_moves[square as usize] = moves;
        }
    }

    fn get_queen_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        self.get_rook_moves(square, occupancy) | self.get_bishop_moves(square, occupancy)
    }

    fn get_bishop_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.bishop_tbl[square as usize].get_index(occupancy);
        self.bishop_moves[index]
    }

    fn get_rook_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.rook_tbl[square as usize].get_index(occupancy);
        self.rook_moves[index]
    }

    pub fn generate_legal_moves(&self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = self.generate_moves(position, side);

        for i in (0..moves.len()).rev() {
            let mv: Move = moves[i];
            position.make_move(mv);

            if self.attacks_to_king(position, side) != EMPTY_BITBOARD {
                moves.remove(i);
            }

            position.unmake();
        }

        moves
    }

    /**
     * Returns a BitBoard that if that shows attacks to king 
     *
     * If the BitBoard is empty then there are no attacks on king, otherwise there are attacks on
     * the:king
     * */
    pub fn attacks_to_king(&self, position: &Position, side: Side) -> BitBoard {
        let king: Square = match side {
            Side::White => position.white_king,
            Side::Black => position.black_king,
        };

        let opponent: Side = match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        let opponent_pawns: BitBoard = position.piece_bitboard(Piece::Pawn, opponent);
        let opponent_knights: BitBoard = position.piece_bitboard(Piece::Knight, opponent);
        let opponent_rooks: BitBoard = position.piece_bitboard(Piece::Rook, opponent);
        let opponent_bishop: BitBoard = position.piece_bitboard(Piece::Bishop, opponent);
        let opponent_queen: BitBoard = position.piece_bitboard(Piece::Queen, opponent);

        (self.get_bishop_moves(king as u64, position.main_bitboard) & opponent_bishop)
            | (self.get_rook_moves(king as u64, position.main_bitboard) & opponent_rooks)
            | (self.knight_moves[king as usize] & opponent_knights)
            | (self.pawn_attacks[side as usize][king as usize] & opponent_pawns)
            | (self.get_queen_moves(king as u64, position.main_bitboard) & opponent_queen)
    }

    fn create_moves(
        &self,
        position: &Position,
        piece: Piece,
        side: Side,
        piece_moves: BitBoard,
        square: Square,
        moves: &mut Vec<Move>,
    ) {
        let mut bb: BitBoard = piece_moves & (!position.main_bitboard);
        while bb != EMPTY_BITBOARD {
            let target: Square = bb.bitscan_forward_reset();
            let rank = match side {
                Side::White => EIGTH_RANK,
                Side::Black => FIRST_RANK,
            };

            let pawn_promotion =
                piece.is_pawn() && (BitBoard(1 << target as usize) & rank) != EMPTY_BITBOARD;

            if pawn_promotion {
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Queen),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Knight),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Bishop),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Rook),
                ));
            } else {
                moves.push(Move::new(piece, square, target, MoveType::Quiet));
            }
        }

        let mut bb_2: BitBoard = if piece.is_pawn() {
            self.pawn_attacks[side as usize][square as usize] & (position.opponent_bitboard())
        } else {
            piece_moves & (position.opponent_bitboard())
        };

        while bb_2 != EMPTY_BITBOARD {
            let target: Square = bb_2.bitscan_forward_reset();
            let rank = match side {
                Side::White => EIGTH_RANK,
                Side::Black => FIRST_RANK,
            };
            let pawn_promotion =
                piece.is_pawn() && (BitBoard(1 << target as usize) & rank) != EMPTY_BITBOARD;

            if pawn_promotion {
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Queen),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Knight),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Bishop),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    target,
                    MoveType::Promotion,
                    Some(Piece::Rook),
                ));
            } else {
                moves.push(Move::new(piece, square, target, MoveType::Capture));
            }
        }
    }

    fn help_gen_sliding_pieces(
        &self,
        piece: Piece,
        piece_bb: BitBoard,
        position: &Position,
        side: Side,
    ) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut bb = piece_bb;
        while bb != EMPTY_BITBOARD {
            let from: Square = bb.bitscan_forward_reset();
            let mut piece_moves = match piece {
                Piece::Rook => self.get_rook_moves(from as u64, position.main_bitboard),
                Piece::Bishop => self.get_bishop_moves(from as u64, position.main_bitboard),
                Piece::Queen => self.get_queen_moves(from as u64, position.main_bitboard),
                _ => panic!("Not a sliding piece"),
            } & !position.side_bitboards[side as usize];

            let p_moves = piece_moves;

            while piece_moves != EMPTY_BITBOARD {
                let target = piece_moves.bitscan_forward_reset();
                let target_bb = BitBoard(1u64 << target as u64);

                if (p_moves & target_bb) & position.opponent_bitboard() != EMPTY_BITBOARD {
                    let mv = Move::new(piece, from, target, MoveType::Capture);
                    moves.push(mv);
                } else {
                    let mv = Move::new(piece, from, target, MoveType::Quiet);
                    moves.push(mv);
                }
            }
        }
        moves
    }

    fn gen_knight_moves(&self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut knights = position.piece_bitboard(Piece::Knight, side);
        let knight_moves = self.knight_moves;

        while knights != EMPTY_BITBOARD {
            let from: Square = knights.bitscan_forward_reset();

            let mut piece_moves =
                knight_moves[from as usize] & !position.side_bitboards[side as usize];
            let p_moves = knight_moves[from as usize] & !position.side_bitboards[side as usize];

            while piece_moves != EMPTY_BITBOARD {
                let target = piece_moves.bitscan_forward_reset();
                let target_bb = BitBoard(1u64 << target as u64);

                if (p_moves & target_bb) & position.opponent_bitboard() != EMPTY_BITBOARD {
                    let mv = Move::new(Piece::Knight, from, target, MoveType::Capture);
                    moves.push(mv);
                } else {
                    let mv = Move::new(Piece::Knight, from, target, MoveType::Quiet);
                    moves.push(mv);
                }
            }
        }

        moves
    }

    fn gen_king_moves(&self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut king: BitBoard = position.piece_bitboard(Piece::King, side);
        let opponent_king_moves: BitBoard = match side {
            Side::Black => self.king_moves[position.white_king as usize],
            Side::White => self.king_moves[position.black_king as usize],
        };

        let from: Square = king.bitscan_forward_reset();
        let mut piece_moves =
            if self.king_moves[from as usize] & opponent_king_moves == EMPTY_BITBOARD {
                self.king_moves[from as usize]
            } else {
                let intersection = self.king_moves[from as usize] & opponent_king_moves;

                self.king_moves[from as usize] & !intersection
            } & !position.side_bitboards[side as usize];

        let p_moves = piece_moves;

        while piece_moves != EMPTY_BITBOARD {
            let target = piece_moves.bitscan_forward_reset();
            let target_bb = BitBoard(1u64 << target as u64);

            if (p_moves & target_bb) & position.opponent_bitboard() != EMPTY_BITBOARD {
                let mv = Move::new(Piece::King, from, target, MoveType::Capture);
                moves.push(mv);
            } else {
                let mv = Move::new(Piece::King, from, target, MoveType::Quiet);
                moves.push(mv);
            }
        }

        let (kingside_castle_king, queenside_castle_king, rank): (Square, Square, BitBoard) =
            match side {
                Side::White => (WHITE_KINGSIDE_KING, WHITE_QUEENSIDE_KING, FIRST_RANK),
                Side::Black => (BLACK_KINGSIDE_KING, BLACK_QUEENSIDE_KING, EIGTH_RANK),
            };

        let castle_rights: (bool, bool) = match side {
            Side::White => (
                position.state.castling_rights.white_king_side(),
                position.state.castling_rights.white_queen_side(),
            ),
            Side::Black => (
                position.state.castling_rights.black_king_side(),
                position.state.castling_rights.black_queen_side(),
            ),
        };

        let kingside_castle = castle_rights.0;
        let queenside_castle = castle_rights.1;

        if kingside_castle {
            let checkers: BitBoard = self.attacks_to_king(position, side);
            let upper = BitBoard(!1 << from as usize);
            let king_side_squares: BitBoard =
                upper & rank & !position.piece_bitboard(Piece::Rook, side);

            let blockers: BitBoard =
                upper & position.main_bitboard & !position.piece_bitboard(Piece::Rook, side) & rank;

            if !self.castle_squares_attacked(position, side, king_side_squares)
                && blockers == EMPTY_BITBOARD
            {
                if position.pieces[kingside_castle_king as usize].is_none()
                    && position.pieces[from as usize + 1].is_none()
                    && checkers == EMPTY_BITBOARD
                {
                    let king_side: Move =
                        Move::new(Piece::King, from, kingside_castle_king, MoveType::Castle);
                    moves.push(king_side);
                }
            }
        }

        if queenside_castle {
            let checkers: BitBoard = self.attacks_to_king(position, side);
            let lower = BitBoard((1 << from as usize) - 1);
            let queen_side_squares: BitBoard =
                lower & rank & !position.piece_bitboard(Piece::Rook, side)
                    ^ BitBoard((1 << queenside_castle_king as usize) >> 1);

            let blockers: BitBoard =
                lower & position.main_bitboard & !position.piece_bitboard(Piece::Rook, side) & rank;

            if !self.castle_squares_attacked(position, side, queen_side_squares)
                && blockers == EMPTY_BITBOARD
                && checkers == EMPTY_BITBOARD
            {
                if position.pieces[queenside_castle_king as usize].is_none()
                    && position.pieces[from as usize - 1].is_none()
                {
                    let queen_side: Move =
                        Move::new(Piece::King, from, queenside_castle_king, MoveType::Castle);
                    moves.push(queen_side);
                }
            }
        }

        moves
    }

    fn gen_en_passant_moves(&self, position: &Position, side: Side) -> Vec<Move> {
        let mut pawns = position.piece_bitboard(Piece::Pawn, side);
        let mut moves = Vec::new();
        while pawns != EMPTY_BITBOARD {
            let from: Square = pawns.bitscan_forward_reset();
            if let Some(ep_sq) = position.state.en_passant {
                let ep_attacks = self.pawn_attacks[side as usize][from as usize]
                    & BitBoard(1u64 << (ep_sq as u64));

                if ep_attacks != EMPTY_BITBOARD {
                    let _target_ep = ep_attacks.bitscan_forward();
                    //log::debug!("EP SQUARE: {:?}", target_ep);
                    let mv = Move::new(Piece::Pawn, from, ep_sq, MoveType::EnPassant);
                    moves.push(mv);
                }
            }
        }
        moves
    }

    pub fn generate_moves(&self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(60);

        let pawn_moves = self.gen_pawn_moves(position);
        let ep_moves = self.gen_en_passant_moves(position, side);

        let knight_moves = self.gen_knight_moves(position, side);

        let rooks = position.piece_bitboard(Piece::Rook, side);
        let rook_moves = self.help_gen_sliding_pieces(Piece::Rook, rooks, position, side);

        let bishop = position.piece_bitboard(Piece::Bishop, side);
        let bishop_moves = self.help_gen_sliding_pieces(Piece::Bishop, bishop, position, side);

        let queen = position.piece_bitboard(Piece::Queen, side);
        let queen_moves = self.help_gen_sliding_pieces(Piece::Queen, queen, position, side);

        let king_moves = self.gen_king_moves(position, side);

        moves.extend(king_moves);
        moves.extend(rook_moves);
        moves.extend(bishop_moves);
        moves.extend(queen_moves);
        moves.extend(knight_moves);
        moves.extend(pawn_moves);
        moves.extend(ep_moves);

        moves
    }
}
