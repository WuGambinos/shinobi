/*use crate::{
    mov::{Move, MoveType},
    Engine, Position, Side,
};

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99999999;
const MAX_DEPTH: i32 = 3;
pub static mut best_move: Option<Move> = None;

pub struct Bot {
    best_move: Option<Move>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot { best_move: None }
    }

    pub fn think(&mut self, engine: &mut Engine) -> Option<Move> {
        self.negamax_alpha_beta(engine, -LARGE_NUM, LARGE_NUM, MAX_DEPTH);
        unsafe {
            log::debug!("BEST_MOVE: {:?}", best_move);
            return best_move;
        }
    }

    pub fn order_moves(&self, engine: &mut Engine, moves: &mut Vec<Move>) {
        moves.sort_by(|a, b| {
            self.score_move(engine, *b)
                .cmp(&self.score_move(engine, *a))
        });
    }

    pub fn score_move(&self, engine: &mut Engine, mv: Move) -> i32 {
        if mv.move_type() == MoveType::Capture {
            let piece_captured = engine.position.pieces[mv.target() as usize]
                .unwrap()
                .1;
            return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
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

        let material_score = white_score + black_score;
        let side_to_move = if position.state.turn() == Side::White {
            1
        } else {
            -1
        };

        return material_score * side_to_move;
    }

    pub fn negamax_alpha_beta(
        &mut self,
        engine: &mut Engine,
        alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {
        if engine.is_draw() {
            return 0;
        }

        let turn = engine.position.state.turn;
        let moves = engine
            .move_gen
            .generate_legal_moves(&mut engine.position, turn);

        if depth == 0 || moves.len() == 0 {
            if engine.checkmate() {
                return -9999999;
            }

            return self.evalutate(&engine.position);
        }

        let mut max_eval = -LARGE_NUM;
        for mv in moves {
            engine.position.make_move(mv);
            let eval = -1 * self.negamax_alpha_beta(engine, -beta, -alpha, depth - 1);
            engine.position.unmake();

            if eval > max_eval {
                max_eval = eval;

                if depth == MAX_DEPTH {
                    log::debug!("MOVE: {:?}", mv);
                    unsafe {
                        best_move = Some(mv);
                    }
                }

                let new_alpha = alpha.max(max_eval);

                if new_alpha >= beta {
                    break;
                }
            }
        }
        return max_eval;
    }
}
*/
