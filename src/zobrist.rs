use crate::{random_u64, MyRng, Piece, Position, Side, NUM_SQUARES};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use strum::IntoEnumIterator;

const SEED: u64 = 12345;

pub struct Zobrist {
    rand_piece_nums: [[u64; NUM_SQUARES as usize]; 12],
    rand_en_passant_nums: [u64; 64],
    rand_castling_rights_num: [u64; 16],
    rand_side_num: u64,
}

impl Zobrist {
    pub fn new() -> Zobrist {
        let mut rand_piece_nums = [[0; NUM_SQUARES as usize]; 12];
        let mut rand_en_passant_nums = [0; 64];
        let mut rand_castling_rights_num = [0; 16];

        let mut rng = ChaChaRng::seed_from_u64(SEED);

        for piece in Piece::iter() {
            for square in 0..NUM_SQUARES as usize {
                rand_piece_nums[piece as usize][square] = rng.gen::<u64>();
            }
        }

        for square in 0..NUM_SQUARES as usize {
            rand_en_passant_nums[square] = rng.gen::<u64>();
        }

        for square in 0..16 {
            rand_castling_rights_num[square] = rng.gen::<u64>();
        }

        let rand_side_num = rng.gen::<u64>();

        Zobrist {
            rand_piece_nums,
            rand_en_passant_nums,
            rand_castling_rights_num,
            rand_side_num,
        }
    }

    pub fn generate_hash_key(&mut self, position: &Position) -> u64 {
        let mut key = 0;

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut piece_bitboard = position.piece_bitboard(piece, side);

                while piece_bitboard.0 > 0 {
                    let square = piece_bitboard.bitscan_forward_reset();
                    key ^= self.rand_piece_nums[piece as usize][square as usize];
                }
            }
        }

        if let Some(ep) = position.state.en_passant_square {
            key ^= self.rand_en_passant_nums[ep as usize];
        }

        if position.state.turn == Side::Black {
            key ^= self.rand_side_num;
        }

        key ^= self.rand_castling_rights_num[position.state.castling_rights.0 as usize];

        return key;
    }

    /*
    pub fn generate_hash_key(&mut self, position: &Position) -> u64 {

        let mut final_key: u64 = 0;

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut piece_bitboard = position.piece_bitboard(piece, side);

                while piece_bitboard.0 > 0 {
                    let square = piece_bitboard.bitscan_forward_reset();
                    final_key ^= self.piece_keys[side as usize][piece as usize][square as usize];
                }
            }
        }

        // EP Hash
        if let Some(ep) = position.state.en_passant_square {
            final_key ^= self.enpassant_keys[ep as usize]
        }

        // Castling Hash
        final_key ^= self.castle_keys[position.state.castling_rights.0 as usize];
        final_key ^= self.sides_key[position.state.turn as usize];

        return final_key;
    }
    */
}
