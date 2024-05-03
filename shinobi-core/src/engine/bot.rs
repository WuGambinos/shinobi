use std::i8::MAX;

use crate::{
    mov::{Move, MoveType},
    MoveGenerator, Position, Side,
};

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99_999_999;
const MAX_DEPTH: i32 = 4;
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

pub struct Bot {
    best_move: Option<Move>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot { best_move: None }
    }

    pub fn think(&mut self, position: &mut Position, move_gen: &MoveGenerator) -> Option<Move> {
        for i in 0..=MAX_DEPTH {
            self.search_position(position, move_gen, -LARGE_NUM, LARGE_NUM, i);
        }
        self.best_move
    }

    pub fn order_moves(&self, position: &Position, moves: &mut [Move]) {
        moves.sort_by(|a, b| {
            self.score_move(position, *b)
                .cmp(&self.score_move(position, *a))
        });
    }

    pub fn score_move(&self, position: &Position, mv: Move) -> i32 {
        if mv.move_type() == MoveType::Capture {
            let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
            //let score = MVV_LVA[piece_captured as usize][mv.piece() as usize] as i32;
            WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10
        } else {
            0
        }
    }

    pub fn evalutate(&self, position: &Position) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        let piece_count = position.piece_count;

        for (i, count) in piece_count[Side::White as usize].iter().enumerate() {
            white_score += WEIGHTS[i] * (*count as i32);
        }

        for (i, count) in piece_count[Side::Black as usize].iter().enumerate() {
            black_score += WEIGHTS[i] * (*count as i32);
        }

        let material_score = white_score - black_score;
        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        material_score * side_to_move
    }

    pub fn search_position(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        alpha: i32,
        beta: i32,
        depth: i32,
    ) {
        self.negamax_alpha_beta(position, move_gen, alpha, beta, depth);
    }
    pub fn negamax_alpha_beta(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {
        if position.is_draw(move_gen) {
            return -2000;
        }

        let turn = position.state.current_turn();
        let mut moves = move_gen.generate_legal_moves(position, turn);

        //self.order_moves(position, &mut moves);

        if depth == 0 || moves.is_empty() {
            if position.checkmate(move_gen) {
                return -9_999_999;
            }

            return self.evalutate(position);
        }

        let mut max_eval = -LARGE_NUM;
        for i in 0..moves.len() { 
            let mv = moves.get(i);
            position.make_move(mv);
            let eval = -self.negamax_alpha_beta(position, move_gen, -beta, -alpha, depth - 1);
            position.unmake();

            if eval > max_eval {
                max_eval = eval;

                if depth == MAX_DEPTH {
                    self.best_move = Some(mv);
                }

                alpha = alpha.max(max_eval);

                if alpha >= beta {
                    break;
                }
            }
        }

        max_eval
    }
}
