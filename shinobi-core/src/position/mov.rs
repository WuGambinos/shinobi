use crate::{square_name, Piece, Side, Square};
use core::fmt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub const MAX_MOVES: usize = 218;
pub const NULL_MOVE: Move = Move(0);
/*
const PIECE_MASK: u32           =   0b00000000000000000000000000000111;
const FROM_MASK: u32            =   0b00000000000000000000000111111000;
const TARGET_MASK: u32          =   0b00000000000000000111111000000000;
const MOVE_TYPE_MASK: u32       =   0b00000000000000111000000000000000;
const PROMOTION_PIECE_MASK: u32 =   0b00000000000011000000000000000000;
*/
const PIECE_MASK: u32 = 0x7;
const FROM_MASK: u32 = 0x1F8;
const TARGET_MASK: u32 = 0x7E00;
const MOVE_TYPE_MASK: u32 = 0x38000;
const PROMO_PIECE_MASK: u32 = 0xC0000;

const FROM_SHIFT: u32 = 3;
const TARGET_SHIFT: u32 = 9;
const MOVE_TYPE_SHIFT: u32 = 15;
const PROMO_SHIFT: u32 = 18;

#[rustfmt::skip]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MoveType {
    Quiet       =   0b0000,
    Capture     =   0b0001,
    EnPassant   =   0b0010,
    Castle      =   0b0011,
    Promotion   =   0b0100,
    All         =   0b0101,
}

impl From<u32> for MoveType {
    fn from(move_type: u32) -> MoveType {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Move(pub u32);

impl Move {
    #[inline(always)]
    pub fn init(piece: Piece, from: Square, target: Square, move_type: MoveType) -> Move {
        let res = ((move_type as u32) << MOVE_TYPE_SHIFT)
            | ((target as u32) << TARGET_SHIFT)
            | ((from as u32) << FROM_SHIFT)
            | (piece as u32);
        Move(res)
    }

    pub fn init_with_promotion_piece(
        piece: Piece,
        from: Square,
        target: Square,
        move_type: MoveType,
        promotion_piece: Piece,
    ) -> Move {
        let promo_piece: u32 = (promotion_piece as i32 - 1) as u32;
        let res = (promo_piece << PROMO_SHIFT)
            | ((move_type as u32) << MOVE_TYPE_SHIFT)
            | ((target as u32) << TARGET_SHIFT)
            | ((from as u32) << FROM_SHIFT)
            | (piece as u32);
        Move(res)
    }

    pub fn is_double_pawn_push(&self) -> bool {
        let piece = self.piece();
        piece.is_pawn() && (self.target() as i8 - self.from() as i8).abs() == 16
    }

    pub fn piece(&self) -> Piece {
        Piece::from(self.0 & PIECE_MASK)
    }

    pub fn from(&self) -> Square {
        let res = (self.0 & FROM_MASK) >> FROM_SHIFT;
        Square::from(res)
    }

    pub fn target(&self) -> Square {
        let res = (self.0 & TARGET_MASK) >> TARGET_SHIFT;
        Square::from(res)
    }

    pub fn move_type(&self) -> MoveType {
        MoveType::from((self.0 & MOVE_TYPE_MASK) >> MOVE_TYPE_SHIFT)
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        if self.move_type() != MoveType::Promotion {
            return None;
        }

        let promotion_piece = (self.0 & PROMO_PIECE_MASK) >> PROMO_SHIFT;
        if (promotion_piece as i32) < 0 {
            None
        } else {
            Some(Piece::from(promotion_piece + 1))
        }
    }

    pub fn print_info(&self) {
        println!(
            "PIECE: {:?}\tFROM: {:?}\tTARGET: {:?}\tMOVE_TYPE: {:?}\tPROMOTION_PIECE: {:?}",
            self.piece(),
            self.from(),
            self.target(),
            self.move_type(),
            self.promotion_piece()
        );
    }
}

/*
#[cfg_attr(
    target_os = "wasm32",
    derive(Debug, Clone, Copy, Serialize, Deserialize)
)]
#[cfg_attr(
    target_os = "wasm32",
    wasm_bindgen)]
#[cfg_attr(not(target_os = "wasm32"), derive(Debug, Clone))]
//#[cfg_attr(not(target_os = "wasm32"), derive(Debug, Clone, Copy))]
pub struct MoveList {
    /*
    #[cfg(target_os = "wasm32")]
    list: Vec<Move>,
    */
    /*
    #[serde(with = "serde_arrays")]
    pub list: [Move; MAX_MOVES],
    */
    /*
    #[cfg(not(target_os = "wasm32"))]
    pub list: [Move; MAX_MOVES],
    */
    pub list: [Move; MAX_MOVES],
    pub count: usize,
}
*/

//#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct MoveList {
    pub list: [Move; MAX_MOVES],
    pub count: usize,
}

/*
#[cfg_attr(
    target_os = "wasm32",
    wasm_bindgen)]
*/
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
