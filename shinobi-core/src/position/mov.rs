use core::fmt;

use crate::{square_name, Piece, Side, SquareLabel};

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
    pub piece: Piece,
    pub from_square: SquareLabel,
    pub target_square: SquareLabel,
    pub move_type: MoveType,
    pub promotion_piece: Option<Piece>,
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

    pub fn is_double_pawn_push(&self) -> bool {
        self.piece.is_pawn() && (self.target_square() as i8 - self.from_square() as i8).abs() == 16
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn from_square(&self) -> SquareLabel {
        self.from_square
    }

    pub fn target_square(&self) -> SquareLabel {
        self.target_square
    }

    pub fn move_type(&self) -> MoveType {
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
