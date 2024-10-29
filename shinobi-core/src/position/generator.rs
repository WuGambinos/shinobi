use crate::{
    castling_rights::BLACK_KINGSIDE_KING, castling_rights::BLACK_QUEENSIDE_KING,
    castling_rights::WHITE_KINGSIDE_KING, castling_rights::WHITE_QUEENSIDE_KING,
    init_slider_attacks, mov::Move, mov::MoveList, mov::MoveType, piece::Piece, square::Square,
    BitBoard, Position, SMagic, Side, A_FILE, B_FILE, EIGTH_RANK, EMPTY_BITBOARD, FIFTH_RANK,
    FIRST_RANK, FOURTH_RANK, G_FILE, H_FILE, NUM_SIDES, NUM_SQUARES, SECOND_RANK, SEVENTH_RANK,
    SIXTH_RANK, THIRD_RANK,
};
use strum::IntoEnumIterator;

// Holds info needed for Move Generation
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

    pub fn gen_pawn_moves(&self, position: &Position, move_type: MoveType, moves: &mut MoveList) {
        let turn = position.state.current_turn();
        let pawns: BitBoard = position.piece_bitboard(Piece::Pawn, turn);

        let seventh_rank = match turn {
            Side::White => SEVENTH_RANK,
            Side::Black => SECOND_RANK,
        };

        let empty = !position.main_bitboard;
        let non_promotions = pawns & !seventh_rank;

        let (single_pushes, third_rank) = match turn {
            Side::White => (self.north_one(non_promotions) & empty, THIRD_RANK),
            Side::Black => (self.south_one(non_promotions) & empty, SIXTH_RANK),
        };

        let double_pushes = match turn {
            Side::White => self.north_one(single_pushes & third_rank) & empty,
            Side::Black => self.south_one(single_pushes & third_rank) & empty,
        };

        if move_type == MoveType::All || move_type == MoveType::Quiet {
            for target in single_pushes {
                let from = match turn {
                    Side::White => target as u64 - 8,
                    Side::Black => target as u64 + 8,
                };
                moves.push(Move::init(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Quiet,
                ));
            }

            for target in double_pushes {
                let from = match turn {
                    Side::White => target as u64 - 16,
                    Side::Black => target as u64 + 16,
                };
                moves.push(Move::init(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Quiet,
                ));
            }

            let promotions = match position.state.current_turn() {
                Side::White => self.north_one(pawns & seventh_rank),
                Side::Black => self.south_one(pawns & seventh_rank),
            } & empty;

            for target in promotions {
                let from = match position.state.current_turn() {
                    Side::White => target as u64 - 8,
                    Side::Black => target as u64 + 8,
                };

                moves.push(Move::init_with_promotion_piece(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Promotion,
                    Piece::Bishop,
                ));
                moves.push(Move::init_with_promotion_piece(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Promotion,
                    Piece::Knight,
                ));
                moves.push(Move::init_with_promotion_piece(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Promotion,
                    Piece::Rook,
                ));
                moves.push(Move::init_with_promotion_piece(
                    Piece::Pawn,
                    Square::from(from),
                    target,
                    MoveType::Promotion,
                    Piece::Queen,
                ));
            }
        }

        if move_type == MoveType::All || move_type == MoveType::Capture {
            let captures = pawns & !seventh_rank;

            for from in captures {
                let attacks =
                    self.pawn_attacks[turn as usize][from as usize] & position.opponent_bitboard();
                for target in attacks {
                    moves.push(Move::init(
                        Piece::Pawn,
                        Square::from(from),
                        target,
                        MoveType::Capture,
                    ));
                }
            }

            // Pawn Promotion Captures
            let promotions = pawns & seventh_rank;

            for from in promotions {
                let captures =
                    position.opponent_bitboard() & self.pawn_attacks[turn as usize][from as usize];
                for target in captures {
                    moves.push(Move::init_with_promotion_piece(
                        Piece::Pawn,
                        Square::from(from),
                        target,
                        MoveType::Promotion,
                        Piece::Bishop,
                    ));
                    moves.push(Move::init_with_promotion_piece(
                        Piece::Pawn,
                        Square::from(from),
                        target,
                        MoveType::Promotion,
                        Piece::Knight,
                    ));
                    moves.push(Move::init_with_promotion_piece(
                        Piece::Pawn,
                        Square::from(from),
                        target,
                        MoveType::Promotion,
                        Piece::Rook,
                    ));
                    moves.push(Move::init_with_promotion_piece(
                        Piece::Pawn,
                        Square::from(from),
                        target,
                        MoveType::Promotion,
                        Piece::Queen,
                    ));
                }
            }
        }
    }

    /*
     * Populates array of bitboards for pawn attacks
     * */
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

    /*
     * Returns BitBoard showing what castle squares are being attacked, if any
     * */
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

    /*
     * Populates array of bitboards for king moves
     * */
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

    /*
     * Populates array of bitboards for knight moves
     * */
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

    /*
     * Returns MoveList containing all legal moves for current side in current position
     * */
    pub fn generate_legal_moves(
        &self,
        position: &mut Position,
        side: Side,
        move_type: MoveType,
    ) -> MoveList {
        let mut moves: MoveList = self.generate_moves(position, side, move_type);

        for i in (0..moves.len()).rev() {
            let mv: Move = moves.get(i);
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
     * the king
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

    fn help_gen_sliding_pieces(
        &self,
        piece: Piece,
        piece_bb: BitBoard,
        position: &Position,
        side: Side,
        move_type: MoveType,
        moves: &mut MoveList,
    ) {
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
                    if move_type == MoveType::Capture || move_type == MoveType::All {
                        let mv = Move::init(piece, from, target, MoveType::Capture);
                        moves.push(mv);
                    }
                } else {
                    if move_type == MoveType::Quiet || move_type == MoveType::All {
                        let mv = Move::init(piece, from, target, MoveType::Quiet);
                        moves.push(mv);
                    }
                }
            }
        }
    }

    pub fn gen_knight_moves(
        &self,
        position: &mut Position,
        side: Side,
        move_type: MoveType,
        moves: &mut MoveList,
    ) {
        let knights = position.piece_bitboard(Piece::Knight, side);

        for from in knights {
            let piece_moves =
                self.knight_moves[from as usize] & !position.side_bitboards[side as usize];

            for target in piece_moves {
                let target_bb = BitBoard(1u64 << target as u64);
                if (piece_moves & target_bb) & position.opponent_bitboard() != EMPTY_BITBOARD {
                    if move_type == MoveType::Capture || move_type == MoveType::All {
                        moves.push(Move::init(Piece::Knight, from, target, MoveType::Capture));
                    }
                } else {
                    if move_type == MoveType::Quiet || move_type == MoveType::All {
                        moves.push(Move::init(Piece::Knight, from, target, MoveType::Quiet));
                    }
                }
            }
        }
    }

    fn gen_king_moves(
        &self,
        position: &mut Position,
        side: Side,
        move_type: MoveType,
        moves: &mut MoveList,
    ) {
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
                if move_type == MoveType::Capture || move_type == MoveType::All {
                    let mv = Move::init(Piece::King, from, target, MoveType::Capture);
                    moves.push(mv);
                }
            } else {
                if move_type == MoveType::Quiet || move_type == MoveType::All {
                    let mv = Move::init(Piece::King, from, target, MoveType::Quiet);
                    moves.push(mv);
                }
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
                        Move::init(Piece::King, from, kingside_castle_king, MoveType::Castle);
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
                        Move::init(Piece::King, from, queenside_castle_king, MoveType::Castle);
                    moves.push(queen_side);
                }
            }
        }
    }

    fn gen_en_passant_moves(
        &self,
        position: &Position,
        side: Side,
        move_type: MoveType,
        moves: &mut MoveList,
    ) {
        if move_type == MoveType::All || move_type == MoveType::Capture {
            let mut pawns = position.piece_bitboard(Piece::Pawn, side);
            while pawns != EMPTY_BITBOARD {
                let from: Square = pawns.bitscan_forward_reset();
                if let Some(ep_sq) = position.state.en_passant {
                    let ep_attacks = self.pawn_attacks[side as usize][from as usize]
                        & BitBoard(1u64 << (ep_sq as u64));

                    if ep_attacks != EMPTY_BITBOARD {
                        let _target_ep = ep_attacks.bitscan_forward();
                        //log::debug!("EP SQUARE: {:?}", target_ep);
                        let mv = Move::init(Piece::Pawn, from, ep_sq, MoveType::EnPassant);
                        moves.push(mv);
                    }
                }
            }
        }
    }

    pub fn generate_moves(
        &self,
        position: &mut Position,
        side: Side,
        move_type: MoveType,
    ) -> MoveList {
        let mut moves: MoveList = MoveList::new();

        self.gen_pawn_moves(position, move_type, &mut moves);
        self.gen_en_passant_moves(position, side, move_type, &mut moves);
        self.gen_knight_moves(position, side, move_type, &mut moves);

        let rooks = position.piece_bitboard(Piece::Rook, side);
        self.help_gen_sliding_pieces(Piece::Rook, rooks, position, side, move_type, &mut moves);

        let bishop = position.piece_bitboard(Piece::Bishop, side);
        self.help_gen_sliding_pieces(Piece::Bishop, bishop, position, side, move_type, &mut moves);

        let queen = position.piece_bitboard(Piece::Queen, side);
        self.help_gen_sliding_pieces(Piece::Queen, queen, position, side, move_type, &mut moves);

        self.gen_king_moves(position, side, move_type, &mut moves);
        moves
    }
}
