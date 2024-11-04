use crate::{
    engine::SearchInfo,
    mov::{Move, MoveList, MoveType, NULL_MOVE},
    piece::Piece,
    pv::PvInfo,
    MoveGenerator, Position, Side,
};

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99_999_999;
const MAX_DEPTH: i32 = 4;
const MATE: i32 = 29000;
pub static mut nodes: i32 = 0;

// MVV_VLA[victim][attacker]
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];

#[derive(Clone)]
pub struct Bot {
    pub searching: Arc<AtomicBool>,
    pub depth: u8,
    pub ply: u8,
    pub nodes: u32,
    pub best_move: Option<Move>,
    pub pv: PvInfo,
    pub killer_moves: [[Move; 64]; 2],
    pub history_moves: [[[i32; 6]; 2]; 64],
}

impl Bot {

    pub fn new() -> Bot {
        Bot {
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

    /* Search for best move in current position 
     * Uses Iterative Deepening and PV
     * Outputs bestmove to stdout in algebraic notation
     * */
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
        let mut d = 1;
        log::info!("SEARCHED STARTED");

        // Iterative Deepening
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

    fn negamax(
        &mut self,
        info: &mut SearchInfo,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {

        // Init PV length
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

        // FIX THIS
        let mut moves_with_scores: Vec<(Move, i32)> = self.score_moves(position, &moves);
        self.order_moves(&position, &mut moves);

        for i in 0..moves.len() {
            self.pick_move(&mut moves_with_scores, i as i32);
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

        let mut captures_with_scores = self.score_moves(position, &mut captures);
        self.order_moves(position, &mut captures);

        for i in 0..captures.len() {
            self.pick_move(&mut captures_with_scores, i as i32);
            let capture = captures.get(i);
            if capture == NULL_MOVE {
                continue;
            }

            self.ply += 1;
            position.make_move(capture);
            let eval = -self.quiescence(info, position, move_gen, -beta, -alpha);
            log::info!("BOT EVAL: {}", eval);
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
            let score = MVV_LVA[piece_captured as usize][mv.piece() as usize] as i32;
            //WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10
            score
        } else {
            0
        }
    }

    fn score_moves(&self, position: &Position, moves: &MoveList) -> Vec<(Move, i32)> {
        let mut moves_with_scores = Vec::with_capacity(30);
        for i in 0..moves.len() {
            let mv = moves.get(i);
            let value = if mv.move_type() == MoveType::Capture {
                let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
                MVV_LVA[piece_captured as usize][mv.piece() as usize] as i32 + 10_000
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

    fn evaluate(&self, position: &Position) -> i32 {
        let piece_count = position.piece_count;
        #[rustfmt::skip]
        let material_score =  
            WEIGHTS[Piece::King as usize] * (piece_count[Side::White as usize][Piece::King as usize] as i32 - piece_count[Side::Black as usize][Piece::King as usize] as i32)
            + WEIGHTS[Piece::Queen as usize] * (piece_count[Side::White as usize][Piece::Queen as usize] as i32 - piece_count[Side::Black as usize][Piece::Queen as usize] as i32)
            + WEIGHTS[Piece::Rook as usize] * (piece_count[Side::White as usize][Piece::Rook as usize] as i32 - piece_count[Side::Black as usize][Piece::Rook as usize] as i32)
            + WEIGHTS[Piece::Bishop as usize] * (piece_count[Side::White as usize][Piece::Bishop as usize] as i32 - piece_count[Side::Black as usize][Piece::Bishop as usize] as i32)
            + WEIGHTS[Piece::Knight as usize] * (piece_count[Side::White as usize][Piece::Knight as usize] as i32 - piece_count[Side::Black as usize][Piece::Knight as usize] as i32)
            + WEIGHTS[Piece::Pawn as usize] * (piece_count[Side::White as usize][Piece::Pawn as usize] as i32 - piece_count[Side::Black as usize][Piece::Pawn as usize] as i32);

        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        return material_score * side_to_move;
    }
}
