use crate::get_time_ms;
use crate::mov::Move;
use crate::mov::MoveType;
use crate::pv::PvInfo;
use crate::MoveGenerator;
use crate::Piece;
use crate::Position;
use crate::SearchInfo;
use crate::Side;
use crate::EMPTY_BITBOARD;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use strum::IntoEnumIterator;

use std::sync::Arc;

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99_999_999;
pub const MAX_DEPTH: i32 = 7;
pub static mut BEST_MOVE: Option<Move> = None;

// PIECE SQUARE TABLES

#[rustfmt::skip]
const PAWNS_SQ: [i32; 64] = [
     0,  0,  0,    0,   0,   0,  0,  0,
    50, 50,  50,  50,  50,  50, 50, 50,
    10, 10,  20,  30,  30,  20, 10, 10,
     5,  5,  10,  25,  25,  10,  5,  5,
     0,  0,   0,  20,  20,   0,  0,  0,
     5, -5, -10,   0,   0, -10, -5,  5,
     5, 10,  10, -20, -20,  10, 10,  5,
     0,  0,   0,   0,   0,   0,  0,  0,
];

#[rustfmt::skip]
const KNIGHT_SQ: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const BISHOP_SQ: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const ROOK_SQ: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];

#[rustfmt::skip]
const QUEEN_SQ: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20,
    -10,   0,   0,  0,  0,   0,   0, -10,
    -10,   0,   5,  5,  5,   5,   0, -10,
     -5,   0,   5,  5,  5,   5,   0,  -5,
      0,   0,   5,  5,  5,   5,   0,  -5,
    -10,   5,   5,  5,  5,   5,   0, -10,
    -10,   0,   5,  0,  0,   0,   0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20,
];

#[rustfmt::skip]
const KING_SQ: [i32; 64] = [
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -20, -30, -30, -40, -40, -30, -30, -20,
  -10, -20, -20, -20, -20, -20, -20, -10, 
   20,  20,   0,   0,   0,   0,  20,  20,
   20,  30,  10,   0,   0,  10,  30,  20
];

// MVV_VLA[victim][attacker]
const MVV_LVA: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];

#[derive(Clone)]
pub struct Search {
    pub searching: Arc<AtomicBool>,
    pub depth: u8,
    pub ply: u8,
    pub nodes: u32,
    pub best_move: Option<Move>,
    pub pv: PvInfo,
}

impl Serialize for Search {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Search", 5)?;
        state.serialize_field("searching", &self.searching.load(Ordering::Relaxed))?;
        state.serialize_field("depth", &self.depth)?;
        state.serialize_field("ply", &self.ply)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("best_move", &self.best_move)?;
        state.end()
    }
}

impl Search {
    pub fn new() -> Search {
        Search {
            searching: Arc::new(AtomicBool::new(false)),
            depth: 0,
            ply: 0,
            nodes: 0,
            best_move: None,
            pv: PvInfo::new(),
        }
    }

    pub fn search_position(
        &mut self,
        info: &mut SearchInfo,
        position: &mut Position,
        move_gen: &MoveGenerator,
        depth: i32,
    ) {
        // Reset
        self.ply = 0;
        self.nodes = 0;
        self.best_move = None;
        log::info!("SEARCHED STARTED");
        let mut d = 1;
        loop {
            let score = self.negamax(info, position, move_gen, -LARGE_NUM, LARGE_NUM, d);
            if self.searching.load(Ordering::Relaxed) {
                print!(
                    "info score cp {} depth {} nodes {} pv",
                    score, d, self.nodes
                );
                for count in 0..self.pv.pv_length[0] {
                    print!(" {}", self.pv.pv_table[0][count as usize].unwrap());
                }
            }
            println!();

            if d == depth {
                break;
            }

            if info.stopped {
                break;
            }
            d += 1;
        }

        if let Some(best_move) = self.best_move {
            log::info!("BEST_MOVE: {:?} NODES: {}", best_move, self.nodes);

            println!("bestmove {}", best_move);
        }
        log::info!("SEARCH ENDED");
    }

    pub fn check(&self, info: &mut SearchInfo) {
        //log::debug!("CHECKING TIME");
        if let Some(stop_time) = info.stop_time {
            if !info.infinite && get_time_ms() > stop_time {
                info.stopped = true;
            }
        }
    }

    #[inline(always)]
    pub fn negamax(
        &mut self,
        info: &mut SearchInfo,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {
        // init PV length
        self.pv.pv_length[self.ply as usize] = self.ply as i32;
        if !self.searching.load(Ordering::Relaxed) {
            return 0;
        }

        if depth == 0 {
            return self.quiescence(info, position, move_gen, alpha, beta);
        }

        if (self.nodes & 2047) == 0 {
            self.check(info);
        }

        self.nodes += 1;
        let mut best_so_far: Option<Move> = None;
        let old_alpha = alpha;
        let mut moves = move_gen.generate_legal_moves(position, position.state.current_turn());

        Search::score_moves(position, &mut moves);
        for i in 0..moves.len() {
            self.pick_move(&mut moves, i as i32);
            let mv = moves[i];
            self.ply += 1;
            position.make_move(mv);
            let score = -self.negamax(info, position, move_gen, -beta, -alpha, depth - 1);
            self.ply -= 1;
            position.unmake();

            if info.stopped {
                return 0;
            }

            // Fail-hard beta cutoff
            if score >= beta {
                // Move is too "good" (fails high)
                // Opponent will avoid this position
                return beta;
            }

            // Better move found
            if score > alpha {
                // PV Move
                alpha = score;

                // Store PV Move
                self.pv.pv_table[self.ply as usize][self.ply as usize] = Some(mv);

                // Copy move from deeper ply
                for next_ply in (self.ply + 1) as i32..self.pv.pv_length[(self.ply + 1) as usize] {
                    self.pv.pv_table[self.ply as usize][next_ply as usize] =
                        self.pv.pv_table[(self.ply + 1) as usize][next_ply as usize];
                }

                // Adjust PV length
                self.pv.pv_length[self.ply as usize] = self.pv.pv_length[(self.ply + 1) as usize];

                let root_move = self.ply == 0;
                if root_move {
                    best_so_far = Some(mv);
                }
            }
        }

        if moves.is_empty() {
            if position.checkmate(move_gen) {
                return -LARGE_NUM + self.ply as i32;
            } else {
                return -100;
                //return self.evalutate(position);
            }
        }

        if old_alpha != alpha {
            self.best_move = best_so_far;
        }

        // Position not good enough (Fails low)
        alpha
    }

    fn quiescence(
        &mut self,
        info: &mut SearchInfo,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        let eval = self.evalutate(position);

        // Fail-hard beta cutoff
        if eval >= beta {
            // Move is too "good" (fails high)
            // Opponent will avoid this position
            return beta;
        }

        // Better Move found is eval > alpha
        // PV Move
        alpha = alpha.max(eval);

        // FIX THIS (Find other way to only get captures)
        let mut captures: Vec<Move> = move_gen
            .generate_legal_moves(position, position.state.current_turn())
            .into_iter()
            .filter(|item| item.move_type() == MoveType::Capture)
            .collect();

        Search::score_moves(position, &mut captures);
        for i in 0..captures.len() {
            self.pick_move(&mut captures, i as i32);
            let capture = captures[i];
            self.ply += 1;
            position.make_move(capture);
            let eval = -self.quiescence(info, position, move_gen, -beta, -alpha);
            self.ply -= 1;
            position.unmake();

            if info.stopped {
                return 0;
            }

            // Fail-hard beta cutoff
            if eval >= beta {
                // Move is too "good" (fails high)
                // Opponent will avoid this position
                return beta;
            }

            // Better Move found is eval > alpha
            // PV Move
            alpha = alpha.max(eval);
        }
        alpha
    }

    /*
    fn order_moves(&self, position: &Position, moves: &mut [Move]) {
        moves.sort_by(|a, b| {
            self.score_move(position, b)
                .cmp(&self.score_move(position, a))
        });
    }

    fn score_move(&self, position: &Position, mv: &mut Move) {
        let value = if mv.move_type() == MoveType::Capture {
            let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
            //return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
            MVV_LVA[piece_captured as usize][mv.piece() as usize]
        } else {
            0
        };

        mv.score = value;
    }
    */

    fn score_moves(position: &Position, moves: &mut Vec<Move>) {
        for i in 0..moves.len() {
            let mv = moves.get_mut(i).unwrap();
            let value = if mv.move_type() == MoveType::Capture {
                let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
                //return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
                MVV_LVA[piece_captured as usize][mv.piece() as usize]
            } else {
                0
            };

            mv.score = value;
        }
    }

    fn pick_move(&self, moves: &mut Vec<Move>, start_index: i32) {
        for i in (start_index + 1) as usize..moves.len() {
            if moves[i].score > moves[start_index as usize].score {
                moves.swap(start_index as usize, i);
            }
        }
    }

    fn evalutate(&self, position: &Position) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut bitboard = position.piece_bitboard(piece, side);

                while bitboard != EMPTY_BITBOARD {
                    let square = bitboard.bitscan_forward_reset();

                    let index = if side == Side::White {
                        let mut rank = square as usize / 8;
                        rank = 7 - rank;

                        let file = square as usize % 8;

                        rank * 8 + file
                    } else {
                        square as usize
                    };

                    match piece {
                        Piece::Pawn => {
                            if side == Side::White {
                                white_score += PAWNS_SQ[index];
                            } else {
                                black_score += PAWNS_SQ[index];
                            }
                        }
                        Piece::Bishop => {
                            if side == Side::White {
                                white_score += BISHOP_SQ[index];
                            } else {
                                black_score += BISHOP_SQ[index];
                            }
                        }
                        Piece::Knight => {
                            if side == Side::White {
                                white_score += KNIGHT_SQ[index];
                            } else {
                                black_score += KNIGHT_SQ[index];
                            }
                        }
                        Piece::Rook => {
                            if side == Side::White {
                                white_score += ROOK_SQ[index];
                            } else {
                                black_score += ROOK_SQ[index];
                            }
                        }
                        Piece::Queen => {
                            if side == Side::White {
                                white_score += QUEEN_SQ[index];
                            } else {
                                black_score += QUEEN_SQ[index];
                            }
                        }
                        Piece::King => {
                            if side == Side::White {
                                white_score += KING_SQ[index];
                            } else {
                                black_score += KING_SQ[index];
                            }
                        }
                    }
                }
            }
        }

        let material_score = white_score - black_score;
        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        material_score * side_to_move
    }
}