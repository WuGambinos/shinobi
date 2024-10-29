use crate::{piece::Piece, Position, Side, square::Square, EMPTY_BITBOARD, NUM_SQUARES};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use strum::IntoEnumIterator;

const SEED: u64 = 12345;

/**
 * Struct used to caclulate zobrist hash
 * */
#[derive(Debug, Clone, Copy)]
pub struct Zobrist {
    rand_piece_nums: [[[u64; NUM_SQUARES]; 6]; 2],
    rand_en_passant_nums: [u64; NUM_SQUARES],
    rand_castling_rights_nums: [u64; 16],
    rand_side_num: u64,
}

impl Serialize for Zobrist {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut res = String::new();
        res.push_str("ZOBRIST");
        serializer.serialize_str(&res)
    }
}

impl Zobrist {
    pub fn new() -> Zobrist {
        let mut rand_piece_nums = [[[0; NUM_SQUARES]; 6]; 2];
        let mut rand_en_passant_nums = [0; 64];
        let mut rand_castling_rights_num = [0; 16];

        let mut rng = ChaChaRng::seed_from_u64(SEED);

        for side in Side::iter() {
            for piece in Piece::iter() {
                for square in 0..NUM_SQUARES {
                    rand_piece_nums[side as usize][piece as usize][square] = rng.gen::<u64>();
                }
            }
        }

        for item in rand_en_passant_nums.iter_mut().take(NUM_SQUARES) {
            *item = rng.gen::<u64>();
        }

        for item in &mut rand_castling_rights_num {
            *item = rng.gen::<u64>();
        }

        let rand_side_num = rng.gen::<u64>();

        Zobrist {
            rand_piece_nums,
            rand_en_passant_nums,
            rand_castling_rights_nums: rand_castling_rights_num,
            rand_side_num,
        }
    }

    pub fn rand_piece_num(&self, side: Side, piece: Piece, square: Square) -> u64 {
        self.rand_piece_nums[side as usize][piece as usize][square as usize]
    }
    pub fn rand_en_passant(&self, square: Square) -> u64 {
        self.rand_en_passant_nums[square as usize]
    }

    pub fn rand_castling_rights_num(&self, castle: u8) -> u64 {
        self.rand_castling_rights_nums[castle as usize]
    }

    pub fn rand_side_num(&self) -> u64 {
        self.rand_side_num
    }

    pub fn generate_hash(&mut self, position: &Position) -> u64 {
        let mut key = 0;

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut piece_bitboard = position.piece_bitboard(piece, side);

                while piece_bitboard != EMPTY_BITBOARD {
                    let square = piece_bitboard.bitscan_forward_reset();
                    key ^= self.rand_piece_nums[side as usize][piece as usize][square as usize];
                }
            }
        }

        if let Some(ep) = position.state.en_passant {
            key ^= self.rand_en_passant_nums[ep as usize];
        }

        if position.state.current_turn == Side::Black {
            key ^= self.rand_side_num;
        }

        key ^= self.rand_castling_rights_nums[position.state.castling_rights.0 as usize];

        key
    }
}

impl Default for Zobrist {
    fn default() -> Self {
        Self::new()
    }
}
