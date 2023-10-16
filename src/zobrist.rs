use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use strum::IntoEnumIterator;
use crate::{random_u64, MyRng, Piece, Position, Side, NUM_SQUARES};


pub struct Zobrist {
    /*
    rng: MyRng,
    piece_keys: [[[u64; NUM_SQUARES as usize]; 12]; 2],
    enpassant_keys: [u64; 64],
    castle_keys: [u64; 16],
    sides_key: [u64; 2],
    */
}

impl Zobrist {
    pub fn new() -> Zobrist {
        Zobrist {}
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
