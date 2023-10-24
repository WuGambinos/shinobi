use crate::{
    init_slider_attacks, BitBoard, Move, MoveType, Piece, Position, SMagic, Side, SquareLabel,
    A_FILE, BLACK_KINGSIDE_KING_SQUARE, BLACK_QUEENSIDE_KING_SQUARE, B_FILE, EIGTH_RANK,
    EMPTY_BITBOARD, FIRST_RANK, G_FILE, H_FILE, WHITE_KINGSIDE_KING_SQUARE,
    WHITE_QUEENSIDE_KING_SQUARE,
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
pub struct MoveGenerator {
    pub knight_moves: [BitBoard; 64],
    pub pawn_pushes: [[BitBoard; 64]; 2],
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub king_moves: [BitBoard; 64],
    pub rook_moves: [BitBoard; 102400],
    pub bishop_moves: [BitBoard; 5248],
    pub bishop_tbl: [SMagic; 64],
    pub rook_tbl: [SMagic; 64],
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        let mut move_gen = MoveGenerator {
            rook_moves: [EMPTY_BITBOARD; 102400],
            bishop_moves: [EMPTY_BITBOARD; 5248],
            knight_moves: [EMPTY_BITBOARD; 64],
            pawn_pushes: [[EMPTY_BITBOARD; 64]; 2],
            pawn_attacks: [[EMPTY_BITBOARD; 64]; 2],
            king_moves: [EMPTY_BITBOARD; 64],
            bishop_tbl: [SMagic::new(0, 0, 0, 0); 64],
            rook_tbl: [SMagic::new(0, 0, 0, 0); 64],
        };

        move_gen.fill_knight_moves();
        move_gen.fill_king_moves();
        move_gen.fill_pawn_attacks(Side::White);
        move_gen.fill_pawn_attacks(Side::Black);
        init_slider_attacks(&mut move_gen, true);
        init_slider_attacks(&mut move_gen, false);

        move_gen
    }

    pub fn north_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard << 8;
    }

    pub fn south_one(&self, bitboard: BitBoard) -> BitBoard {
        return bitboard >> 8;
    }

    pub fn east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 1) & !A_FILE;
    }

    pub fn west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 1) & !H_FILE;
    }

    pub fn north_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 9) & !A_FILE;
    }

    pub fn north_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 7) & !H_FILE;
    }

    pub fn south_east_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 7) & !A_FILE;
    }

    pub fn south_west_one(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 9) & !H_FILE;
    }

    pub fn north_north_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 17) & !A_FILE;
    }

    pub fn north_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 10) & !(A_FILE | B_FILE);
    }

    pub fn south_east_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 6) & !(A_FILE | B_FILE);
    }

    pub fn south_south_east(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 15) & !A_FILE;
    }

    pub fn north_north_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 15) & !H_FILE;
    }

    pub fn north_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard << 6) & !(G_FILE | H_FILE);
    }

    pub fn south_west_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 10) & !(G_FILE | H_FILE);
    }

    pub fn south_south_west(&self, bitboard: BitBoard) -> BitBoard {
        return (bitboard >> 17) & !H_FILE;
    }

    pub fn white_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        return self.north_one(bitboard) & position.empty_bitboard;
    }

    pub fn white_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        const RANK4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
        let single_pushes = self.white_single_push_target(position, bitboard);
        return self.north_one(single_pushes) & position.empty_bitboard & RANK4;
    }

    pub fn white_pawns_able_push(&self, position: &Position, empty: BitBoard) -> BitBoard {
        return self.south_one(empty)
            & position.piece_bitboards[Side::White as usize][Piece::Pawn as usize];
    }

    pub fn white_pawns_able_double_push(&self, position: &Position) -> BitBoard {
        const RANK4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
        let empty_rank_3 =
            self.south_one(position.empty_bitboard & RANK4) & position.empty_bitboard;
        return self.white_pawns_able_push(position, empty_rank_3);
    }

    pub fn black_single_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        return self.south_one(bitboard) & position.empty_bitboard;
    }

    pub fn black_double_push_target(&self, position: &Position, bitboard: BitBoard) -> BitBoard {
        const RANK5: BitBoard = BitBoard(0x0000_00FF_0000_0000);
        let single_pushes = self.black_single_push_target(position, bitboard);
        return self.south_one(single_pushes) & position.empty_bitboard & RANK5;
    }

    pub fn fill_pawn_pushes(&mut self, position: &Position, side: Side) {
        for square in SquareLabel::iter() {
            let mut pushes: BitBoard = EMPTY_BITBOARD;

            match side {
                Side::White => {
                    let mut white_pawns: BitBoard = EMPTY_BITBOARD;
                    white_pawns.set_bit(square);
                    let single_push = self.white_single_push_target(position, white_pawns);
                    let double_push = self.white_double_push_target(position, white_pawns);
                    pushes |= single_push;
                    if single_push != EMPTY_BITBOARD {
                        pushes |= double_push;
                    }
                    self.pawn_pushes[side as usize][square as usize] = pushes;
                }
                Side::Black => {
                    let mut black_pawns: BitBoard = EMPTY_BITBOARD;
                    black_pawns.set_bit(square);
                    pushes |= self.black_single_push_target(position, black_pawns);
                    pushes |= self.black_double_push_target(position, black_pawns);
                    self.pawn_pushes[side as usize][square as usize] = pushes;
                }
            };
        }
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

    pub fn fill_pawn_attacks(&mut self, side: Side) {
        for square in SquareLabel::iter() {
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

    pub fn fill_pawn_moves(&mut self, position: &Position, side: Side) {
        self.fill_pawn_pushes(position, side);
        self.fill_pawn_attacks(side);
    }

    pub fn attacks_to_king(&self, position: &Position, side: Side) -> BitBoard {
        let king_square: SquareLabel = match side {
            Side::White => position.white_king_square,
            Side::Black => position.black_king_square,
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

        return (self.get_bishop_moves(king_square as u64, position.main_bitboard)
            & opponent_bishop)
            | (self.get_rook_moves(king_square as u64, position.main_bitboard) & opponent_rooks)
            | (self.knight_moves[king_square as usize] & opponent_knights)
            | (self.pawn_attacks[side as usize][king_square as usize] & opponent_pawns)
            | (self.get_queen_moves(king_square as u64, position.main_bitboard) & opponent_queen);
    }

    pub fn castle_squares_attacked(
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

        while bb.0 > 0 {
            let square = SquareLabel::from(bb.bitscan_forward_reset());
            result_board |= (self.get_bishop_moves(square as u64, position.main_bitboard)
                & opponent_bishop)
                | (self.get_rook_moves(square as u64, position.main_bitboard) & opponent_rooks)
                | (self.knight_moves[square as usize] & opponent_knights)
                | (self.pawn_attacks[side as usize][square as usize] & opponent_pawns)
                | (self.get_queen_moves(square as u64, position.main_bitboard) & opponent_queen);
        }
        return result_board != EMPTY_BITBOARD;
    }

    pub fn fill_king_moves(&mut self) {
        for square in SquareLabel::iter() {
            let mut moves: BitBoard = EMPTY_BITBOARD;
            let mut bitboard: BitBoard = EMPTY_BITBOARD;

            bitboard.set_bit(square);

            moves |= self.east_one(bitboard) | self.west_one(bitboard);
            bitboard |= moves;
            moves |= self.north_one(bitboard) | self.south_one(bitboard);

            self.king_moves[square as usize] = moves;
        }
    }

    pub fn fill_knight_moves(&mut self) {
        for square in SquareLabel::iter() {
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
    pub fn get_bishop_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.bishop_tbl[square as usize].get_index(occupancy);
        return self.bishop_moves[index];
    }

    pub fn get_rook_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        let index = self.rook_tbl[square as usize].get_index(occupancy);
        return self.rook_moves[index];
    }

    pub fn get_queen_moves(&self, square: u64, occupancy: BitBoard) -> BitBoard {
        return self.get_rook_moves(square, occupancy) | self.get_bishop_moves(square, occupancy);
    }

    #[inline(always)]
    pub fn create_moves(
        &mut self,
        position: &Position,
        piece: Piece,
        side: Side,
        piece_moves: BitBoard,
        square: SquareLabel,
        moves: &mut Vec<Move>,
    ) {
        let mut bb: BitBoard = piece_moves & (!position.main_bitboard);
        while bb.0 > 0 {
            let to_square = SquareLabel::from(bb.bitscan_forward_reset());
            let rank = match side {
                Side::White => EIGTH_RANK,
                Side::Black => FIRST_RANK,
            };

            let pawn_promotion =
                piece.is_pawn() && (BitBoard(1 << to_square as usize) & rank) != EMPTY_BITBOARD;

            if pawn_promotion {
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Queen),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Knight),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Bishop),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Rook),
                ));
            } else {
                moves.push(Move::new(piece, square, to_square, MoveType::Quiet));
            }
        }

        let mut bb_2: BitBoard = if piece.is_pawn() {
            self.pawn_attacks[side as usize][square as usize] & (position.opponent_bitboard())
        } else {
            piece_moves & (position.opponent_bitboard())
        };

        while bb_2.0 > 0 {
            let to_square = SquareLabel::from(bb_2.bitscan_forward_reset());
            let rank = match side {
                Side::White => EIGTH_RANK,
                Side::Black => FIRST_RANK,
            };
            let pawn_promotion =
                piece.is_pawn() && (BitBoard(1 << to_square as usize) & rank) != EMPTY_BITBOARD;

            if pawn_promotion {
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Queen),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Knight),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Bishop),
                ));
                moves.push(Move::with_promotion_piece(
                    piece,
                    square,
                    to_square,
                    MoveType::Promotion,
                    Some(Piece::Rook),
                ));
            } else {
                moves.push(Move::new(piece, square, to_square, MoveType::Capture));
            }
        }
    }

    pub fn generate_legal_moves(&mut self, position: &mut Position, side: Side) -> Vec<Move> {
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

    pub fn generate_pawn_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let en_passant = position.check_en_passant(square, side);
        let pawn_pushes = self.pawn_pushes[side as usize][square as usize];

        if en_passant.is_some() {
            let mv = Move::new(
                Piece::Pawn,
                square,
                position.state.en_passant_square.unwrap(),
                MoveType::EnPassant,
            );
            moves.push(mv);
        }
        self.create_moves(position, Piece::Pawn, side, pawn_pushes, square, &mut moves);

        moves
    }

    pub fn generate_knight_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let knight_moves = self.knight_moves[square as usize];
        self.create_moves(
            position,
            Piece::Knight,
            side,
            knight_moves,
            square,
            &mut moves,
        );

        moves
    }

    pub fn generate_queen_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let queen_moves = self.get_queen_moves(square as u64, position.main_bitboard);
        self.create_moves(
            position,
            Piece::Queen,
            side,
            queen_moves,
            square,
            &mut moves,
        );

        moves
    }

    pub fn generate_rook_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let rook_moves = self.get_rook_moves(square as u64, position.main_bitboard);
        self.create_moves(position, Piece::Rook, side, rook_moves, square, &mut moves);

        moves
    }

    pub fn generate_bishop_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let bishop_moves = self.get_bishop_moves(square as u64, position.main_bitboard);
        self.create_moves(
            position,
            Piece::Bishop,
            side,
            bishop_moves,
            square,
            &mut moves,
        );
        moves
    }

    pub fn generate_king_moves(
        &mut self,
        position: &mut Position,
        side: Side,
        square: SquareLabel,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let opponent_king_moves: BitBoard = match side {
            Side::Black => self.king_moves[position.white_king_square as usize],
            Side::White => self.king_moves[position.black_king_square as usize],
        };

        let piece_type: Piece = Piece::King;

        let king_moves = if self.king_moves[square as usize] & opponent_king_moves == EMPTY_BITBOARD
        {
            self.king_moves[square as usize]
        } else {
            let intersection = self.king_moves[square as usize] & opponent_king_moves;

            self.king_moves[square as usize] & !intersection
        };

        let (kingside_castle_king_square, queenside_castle_king_square, rank) = match side {
            Side::White => (
                WHITE_KINGSIDE_KING_SQUARE,
                WHITE_QUEENSIDE_KING_SQUARE,
                FIRST_RANK,
            ),
            Side::Black => (
                BLACK_KINGSIDE_KING_SQUARE,
                BLACK_QUEENSIDE_KING_SQUARE,
                EIGTH_RANK,
            ),
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
            let upper = BitBoard(!1 << square as usize);
            let king_side_squares: BitBoard =
                upper & rank & !position.piece_bitboard(Piece::Rook, side);

            let blockers: BitBoard =
                upper & position.main_bitboard & !position.piece_bitboard(Piece::Rook, side) & rank;

            if !self.castle_squares_attacked(position, side, king_side_squares)
                && blockers == EMPTY_BITBOARD
            {
                if position.pieces[kingside_castle_king_square as usize].is_none()
                    && position.pieces[square as usize + 1].is_none()
                    && checkers == EMPTY_BITBOARD
                {
                    let king_side: Move = Move::new(
                        piece_type,
                        square,
                        kingside_castle_king_square,
                        MoveType::Castle,
                    );
                    moves.push(king_side);
                }
            }
        }

        if queenside_castle {
            let checkers: BitBoard = self.attacks_to_king(position, side);
            let lower = BitBoard((1 << square as usize) - 1);
            let queen_side_squares: BitBoard =
                lower & rank & !position.piece_bitboard(Piece::Rook, side)
                    ^ BitBoard((1 << queenside_castle_king_square as usize) >> 1);

            let blockers: BitBoard =
                lower & position.main_bitboard & !position.piece_bitboard(Piece::Rook, side) & rank;

            if !self.castle_squares_attacked(position, side, queen_side_squares)
                && blockers == EMPTY_BITBOARD
                && checkers == EMPTY_BITBOARD
            {
                if position.pieces[queenside_castle_king_square as usize].is_none()
                    && position.pieces[square as usize - 1].is_none()
                {
                    let queen_side: Move = Move::new(
                        piece_type,
                        square,
                        queenside_castle_king_square,
                        MoveType::Castle,
                    );
                    moves.push(queen_side);
                }
            }
        }

        self.create_moves(position, piece_type, side, king_moves, square, &mut moves);
        moves
    }

    pub fn generate_moves(&mut self, position: &mut Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(60);

        self.fill_pawn_moves(position, side);

        for square in SquareLabel::iter() {
            let piece: Option<(Side, Piece)> = position.pieces[square as usize];
            if let Some(p) = piece {
                let piece_type: Piece = p.1;
                let piece_side: Side = p.0;

                if piece_side == side {
                    match piece_type {
                        Piece::Pawn => {
                            moves.extend(self.generate_pawn_moves(position, side, square))
                        }
                        Piece::Knight => {
                            moves.extend(self.generate_knight_moves(position, side, square))
                        }
                        Piece::Queen => {
                            moves.extend(self.generate_queen_moves(position, side, square))
                        }
                        Piece::Rook => {
                            moves.extend(self.generate_rook_moves(position, side, square))
                        }
                        Piece::Bishop => {
                            moves.extend(self.generate_bishop_moves(position, side, square))
                        }
                        Piece::King => {
                            moves.extend(self.generate_king_moves(position, side, square))
                        }
                    }
                }
            }
        }
        moves
    }
}