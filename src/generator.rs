use std::time::Instant;

use crate::{
    get_bishop_attacks, get_queen_attacks, get_rook_attacks, BitBoard, Move, Piece, Position,
    SMagic, Side, SquareLabel, A_FILE, B_FILE, EMPTY_BITBOARD, F_FILE, G_FILE, H_FILE,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

        move_gen
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

    pub fn generate_pawn_pushes(&mut self, position: &Position, side: Side) {
        for square in SquareLabel::iter() {
            let mut pushes: BitBoard = EMPTY_BITBOARD;

            match side {
                Side::White => {
                    let mut white_pawns: BitBoard = EMPTY_BITBOARD;
                    white_pawns.set_bit(square);
                    pushes |= self.white_single_push_target(position, white_pawns);
                    pushes |= self.white_double_push_target(position, white_pawns);

                    /*
                    if self
                        .white_pawns_able_push(self.empty_bitboard)
                        .get_bit(square as u64)
                        == 1
                    {
                        pushes |= self.white_single_push_target(white_pawns);
                    }

                    if self.white_pawns_able_double_push().get_bit(square as u64) == 1 {
                        pushes |= self.white_double_push_target(white_pawns);
                    }
                    */
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

    pub fn generate_pawn_attacks(&mut self, side: Side) {
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

    pub fn generate_pawn_moves(&self, side: Side, square: SquareLabel) -> BitBoard {
        //self.generate_pawn_pushes(side, square) | self.generate_pawn_attacks(side, square)
        todo!();
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

    pub fn generate_bishop_moves(&self, position: &Position, square: SquareLabel) -> BitBoard {
        return BitBoard(get_bishop_attacks(
            position,
            square as u64,
            position.main_bitboard.0,
        )) & !position.side_bitboards[position.state.turn as usize];
    }

    pub fn generate_rook_moves(&self, position: &Position, square: SquareLabel) -> BitBoard {
        return BitBoard(get_rook_attacks(
            position,
            square as u64,
            position.main_bitboard.0,
        )) & !position.side_bitboards[position.state.turn as usize];
    }

    pub fn generate_queen_moves(&self, position: &Position, square: SquareLabel) -> BitBoard {
        return BitBoard(get_queen_attacks(
            position,
            square as u64,
            position.main_bitboard.0,
        )) & !position.side_bitboards[position.state.turn as usize];
    }

    pub fn generate_targets(&mut self, position: &Position, side: Side) {
        self.generate_pawn_pushes(position, side);
    }

    pub fn generate_moves(&mut self, position: &Position, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        self.generate_targets(position, side);
        for square in SquareLabel::iter() {
            let piece: Option<Piece> = position.get_piece_on_square(square, side);
            if let Some(p) = piece {
                match p {
                    Piece::Pawn => {
                        let mut n: u64 = (self.pawn_pushes[side as usize][square as usize]
                            & (!position.main_bitboard))
                            .0;

                        let mut i = 0;
                        while n > 0 {
                            let bit = n & 1;
                            if bit == 1 {
                                moves.push(Move::new(p, square, SquareLabel::from(i)));
                            }

                            n = n >> 1;
                            i += 1;
                        }
                    }
                    Piece::Knight => {
                        let mut n: u64 =
                            (self.knight_moves[square as usize] & (!position.main_bitboard)).0;

                        let mut i = 0;
                        while n > 0 {
                            let bit = n & 1;
                            if bit == 1 {
                                moves.push(Move::new(p, square, SquareLabel::from(i)));
                            }

                            n = n >> 1;
                            i += 1;
                        }
                    }
                    Piece::King => {}
                    _ => (),
                }
            }
        }
        moves
    }
}
