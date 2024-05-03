use crate::{square_name, Piece, Side, Square};
use core::fmt;
use modular_bitfield::prelude::*;
use serde::{Deserialize, Serialize};

const MAX_MOVES: usize = 218;
const NULL_MOVE: Move = Move::new();

#[rustfmt::skip]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MoveType {
    Quiet       =   0b0000,
    Capture     =   0b0001,
    EnPassant   =   0b0010,
    Castle      =   0b0011,
    Promotion   =   0b0100,
}

impl From<u8> for MoveType {
    fn from(move_type: u8) -> MoveType {
        match move_type {
            0b0000 => MoveType::Quiet,
            0b0001 => MoveType::Capture,
            0b0010 => MoveType::EnPassant,
            0b0011 => MoveType::Castle,
            0b0100 => MoveType::Promotion,
            _ => panic!("NOT A MOVE TYPE"),
        }
    }
}

#[modular_bitfield::bitfield]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    piece_bf: B3,
    from_bf: B6,
    target_bf: B6,
    move_type_bf: B3,
    promotion_piece_bf: B3,
    nothing: B3,
}

impl Move {
    pub fn init(piece: Piece, from: Square, target: Square, move_type: MoveType) -> Move {
        Move::new()
            .with_piece_bf(piece as u8)
            .with_from_bf(from as u8)
            .with_target_bf(target as u8)
            .with_move_type_bf(move_type as u8)
            .with_promotion_piece_bf(0)
    }

    pub fn init_with_promotion_piece(
        piece: Piece,
        from: Square,
        target: Square,
        move_type: MoveType,
        promotion_piece: Piece,
    ) -> Move {
        Move::new()
            .with_piece_bf(piece as u8)
            .with_from_bf(from as u8)
            .with_target_bf(target as u8)
            .with_move_type_bf(move_type as u8)
            .with_promotion_piece_bf((promotion_piece as u8) + 1)
    }

    pub fn is_double_pawn_push(&self) -> bool {
        let piece = self.piece();
        piece.is_pawn() && (self.target() as i8 - self.from() as i8).abs() == 16
    }

    pub fn piece(&self) -> Piece {
        Piece::from(self.piece_bf())
    }

    pub fn from(&self) -> Square {
        Square::from(self.from_bf())
    }

    pub fn target(&self) -> Square {
        Square::from(self.target_bf())
    }

    pub fn move_type(&self) -> MoveType {
        MoveType::from(self.move_type_bf())
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        if self.promotion_piece_bf() == 0 {
            None
        } else {
            Some(Piece::from(self.promotion_piece_bf() - 1))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MoveList {
    pub list: [Move; MAX_MOVES],
    pub count: usize,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            list: [NULL_MOVE; MAX_MOVES],
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get(&self, index: usize) -> Move {
        self.list[index]
    }

    pub fn clear(&mut self) {
        for i in 0..self.count {
            self.list[i] = NULL_MOVE;
        }
        self.count = 0;
    }

    pub fn remove(&mut self, index: usize) {
        if index >= self.count {
            panic!("MOVELIST: OUT OF BOUNDS ERROR");
        }

        self.list[index] = NULL_MOVE;
        for i in index..self.count - 1 {
            self.list[i] = self.list[i + 1];
            self.list[i + 1] = NULL_MOVE;
        }

        self.count -= 1;
    }

    pub fn push(&mut self, mv: Move) {
        self.list[self.count] = mv;
        self.count += 1;
    }

    pub fn pop(&mut self) -> Move {
        let res = self.list[self.count];
        self.list[self.count] = NULL_MOVE;
        self.count -= 1;

        res
    }
}

/*
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub piece: Piece,
    pub from: Square,
    pub target: Square,
    pub move_type: MoveType,
    pub promotion_piece: Option<Piece>,
    pub score: i32,
}

impl Move {
    pub fn init(piece: Piece, from: Square, target: Square, move_type: MoveType) -> Move {
        Move {
            piece,
            from,
            target,
            move_type,
            promotion_piece: None,
            score: 0,
        }
    }

    pub fn init_with_promotion_piece(
        piece: Piece,
        from: Square,
        target: Square,
        move_type: MoveType,
        promotion_piece: Option<Piece>,
    ) -> Move {
        Move {
            piece,
            from,
            target,
            move_type,
            promotion_piece,
            score: 0,
        }
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.piece.is_pawn() && (self.target() as i8 - self.from() as i8).abs() == 16
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn from(&self) -> Square {
        self.from
    }

    pub fn target(&self) -> Square {
        self.target
    }

    pub fn move_type(&self) -> MoveType {
        self.move_type
    }
}
*/

/**
 * Prints Move in format like "a1b2"
 * Where a1 is the from square and b2 is the target_square
 */
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.move_type() == MoveType::Promotion {
            // promotion move format like "c7c8q" (pawn from c7 to c8  and promoting to queen)
            write!(
                f,
                "{}{}{}",
                square_name(self.from() as u8),
                square_name(self.target() as u8),
                self.promotion_piece().unwrap().to_char(Side::Black)
            )
        } else {
            write!(
                f,
                "{}{}",
                square_name(self.from() as u8),
                square_name(self.target() as u8)
            )
        }
    }
}
