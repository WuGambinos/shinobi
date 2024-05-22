use crate::get_time_ms;
use crate::mov::Move;
use crate::mov::MoveList;
use crate::mov::MoveType;
use crate::mov::NULL_MOVE;
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
use std::sync::Arc;
use strum::IntoEnumIterator;

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 30000;
const MATE: i32 = 29000;
pub const MAX_DEPTH: i32 = 7;
pub static mut BEST_MOVE: Option<Move> = None;

// PIECE SQUARE TABLES
//
// PSQT FORMAT
/*
 * A8  B8  C8  D8  E8  F8  G8  H8
 * A7  B7  C7  D7  E7  F7  G7  H7
 * A6  B6  C6  D6  E6  F6  G6  H6
 * A5  B5  C5  D5  E5  F5  G5  H5
 * A4  B4  C4  D4  E4  F4  G4  H4
 * A3  B3  C3  D3  E3  F3  G3  H3
 * A2  B2  C2  D2  E2  F2  G2  H2
 * A1  B1  C1  D1  E1  F1  G1  H1
 */

#[rustfmt::skip]
const FLIP: [usize; 64]  = [
    56,  57,  58,  59,  60,  61,  62,  63,
    48,  49,  50,  51,  52,  53,  54,  55,
    40,  41,  42,  43,  44,  45,  46,  47,
    32,  33,  34,  35,  36,  37,  38,  39,
    24,  25,  26,  27,  28,  29,  30,  31,
    16,  17,  18,  19,  20,  21,  22,  23,
    8,   9,   10,  11,  12,  13,  14,  15,
    0,   1,   2,   3,   4,   5,   6,   7
];

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
const MVV_LVA: [[i32; 6]; 6] = [
    [105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600],
];

#[derive(Clone)]
pub struct Search {
    pub searching: Arc<AtomicBool>,
    pub depth: u8,
    pub ply: u8,
    pub nodes: u32,
    pub best_move: Option<Move>,
    pub pv: PvInfo,
    pub killer_moves: [[Move; 64]; 2],
    pub history_moves: [[[i32; 6]; 2]; 64],
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
            killer_moves: [[NULL_MOVE; 64]; 2],
            history_moves: [[[0; 6]; 2]; 64],
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

        /*
        if (self.nodes & 2047) == 0 {
            self.check(info);
        }
        */

        self.nodes += 1;
        let mut best_so_far: Option<Move> = None;
        let old_alpha = alpha;
        let mut moves =
            move_gen.generate_legal_moves(position, position.state.current_turn(), MoveType::All);

        // let mut moves_with_scores: Vec<(Move, i32)> = self.score_moves(position, &moves);
        self.order_moves(&position, &mut moves);

        for i in 0..moves.len() {
            //self.pick_move(&mut moves_with_scores, i as i32);
            let mv = moves.get(i);

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
                // Store Killer moves
                if mv.move_type() != MoveType::Capture {
                    self.killer_moves[1][self.ply as usize] =
                        self.killer_moves[0][self.ply as usize];
                    self.killer_moves[0][self.ply as usize] = mv;
                }

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
                return -MATE + self.ply as i32;
            } else {

                //return -100;
                return 0;
            }
        }

        if old_alpha != alpha {
            self.best_move = best_so_far;
        }

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
        self.nodes += 1;
        let eval = self.evaluate(position);

        // Fail-hard beta cutoff
        if eval >= beta {
            // Move is too "good" (fails high)
            // Opponent will avoid this position
            return beta;
        }

        // Better Move found is eval > alpha
        // PV Move
        alpha = alpha.max(eval);

        let mut captures: MoveList = move_gen.generate_legal_moves(
            position,
            position.state.current_turn(),
            MoveType::Capture,
        );

        // let mut captures_with_scores = self.score_moves(position, &mut captures);
        self.order_moves(position, &mut captures);

        for i in 0..captures.len() {
            //self.pick_move(&mut captures_with_scores, i as i32);
            let capture = captures.get(i);
            if capture == NULL_MOVE {
                continue;
            }

            self.ply += 1;
            position.make_move(capture);
            let eval = -self.quiescence(info, position, move_gen, -beta, -alpha);
            log::info!("EVAL: {}", eval);
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

    fn order_moves(&self, position: &Position, moves: &mut MoveList) {
        let len = moves.len();
        moves.list[0..len].sort_by(|a, b| {
            self.score_move(position, b)
                .cmp(&self.score_move(position, a))
        });
    }

    fn score_move(&self, position: &Position, mv: &Move) -> i32 {
        if mv.move_type() == MoveType::Capture {
            let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
            // MVV_LVA[piece_captured as usize][mv.piece() as usize] + 200
            //let res = (WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10).abs();
            //res
            //MVV_LVA[piece_captured as usize][mv.piece() as usize] as i32
            MVV_LVA[mv.piece() as usize][piece_captured as usize] as i32
        } else {
            /*
            // score 1st killer move
            if self.killer_moves[0][self.ply as usize] == *mv {
                log::info!("FIRST KILLER");
                9000
            }
            // score 2nd killer move
            else if self.killer_moves[1][self.ply as usize] == *mv {
                log::info!("SECOND KILLER");
                8000
            }
            // score history move
            else {
                log::info!("HISTORY");
                self.history_moves[mv.target() as usize][position.state.current_turn() as usize]
                    [mv.piece() as usize]
            }
            */
            0
        }
    }

    /*
    fn score_moves(&self, position: &Position, moves: &MoveList) -> Vec<(Move, i32)> {
        let mut moves_with_scores = Vec::with_capacity(30);
        for i in 0..moves.len() {
            let mv = moves.get(i);
            let value = if mv.move_type() == MoveType::Capture {
                let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
                //return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
                MVV_LVA[piece_captured as usize][mv.piece() as usize] + 10_000
            }
            // Score quiet move
            else {
                // score 1st killer move
                if self.killer_moves[0][self.ply as usize] == mv {
                    9000
                }
                // score 2nd killer move
                else if self.killer_moves[1][self.ply as usize] == mv {
                    8000
                }
                // score history move
                else {
                    self.history_moves[mv.target() as usize][position.state.current_turn() as usize]
                        [mv.piece() as usize]
                }
            };
            moves_with_scores.push((mv, value));
        }

        return moves_with_scores;
    }

    fn pick_move(&self, moves_with_scores: &mut Vec<(Move, i32)>, start_index: i32) {
        for i in (start_index + 1) as usize..moves_with_scores.len() {
            if moves_with_scores[i].1 > moves_with_scores[start_index as usize].1 {
                moves_with_scores.swap(start_index as usize, i);
            }
        }
    }
    */

    fn evaluate(&self, position: &Position) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut bitboard = position.piece_bitboard(piece, side);

                while bitboard != EMPTY_BITBOARD {
                    let square = bitboard.bitscan_forward_reset();

                    let index = if side == Side::White {
                        FLIP[square as usize]
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

        if position.state.current_turn() == Side::White {
            material_score
        } else {
            -material_score
        }
    }
}
