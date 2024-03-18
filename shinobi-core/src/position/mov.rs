use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{square_name, Piece, Side, Square};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MoveType {
    EnPassant,
    Quiet,
    Capture,
    Castle,
    Promotion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub piece: Piece,
    pub from: Square,
    pub target: Square,
    pub move_type: MoveType,
    pub promotion_piece: Option<Piece>,
}

impl Move {
    pub fn new(piece: Piece, from: Square, target: Square, move_type: MoveType) -> Move {
        Move {
            piece,
            from,
            target,
            move_type,
            promotion_piece: None,
        }
    }

    pub fn with_promotion_piece(
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

/**
 * Prints Move in format like "a1b2"
 * Where a1 is the from square and b2 is the target_square
 */
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.move_type == MoveType::Promotion {
            // promotion move format like "c7c8q" (pawn from c7 to c8  and promoting to queen)
            write!(
                f,
                "{}{}{}",
                square_name(self.from() as u8),
                square_name(self.target() as u8),
                self.promotion_piece.unwrap().to_char(Side::Black)
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
