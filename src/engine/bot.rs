use crate::{Move, MoveGenerator, Position};

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99999999;
const MAX_DEPTH: i32 = 3;

pub struct Bot {
    best_move: Option<Move>,
}

/*
impl Bot {
    pub fn new() -> Bot {
        Bot { best_move: None }
    }

    pub fn think(&self) {
        return self.best_move;
    }

    pub fn evalutate(&self, position: &Position) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        let piece_list = None;

        let pl = position.pieces;

        let best_move: Option<Move> = None;

        for i in 0..(WEIGHTS.len() * 2) {
            if (i < 6) {
                white_score += (WEIGHTS[i]);
            } else {
                black_score += (WEIGHTS[i - 6]);
            }
        }
        0
    }

    pub fn negamax_alpha_beta(
        &mut self,
        position: &mut Position,
        move_gen: &mut MoveGenerator,
        alpha: i64,
        beta: i64,
        depth: i32,
    ) {
        // Check for draw

        let moves = move_gen.generate_legal_moves(position, position.state.turn);

        if (depth == 0 || moves.len() == 0) {
            return self.evalutate(position);
        }

        let mut max_eval = -LARGE_NUM;
        for mv in moves {
            position.make_move(mv);
            let eval = -self.negamax_alpha_beta(position, move_gen, -beta, -alpha, depth - 1);
            position.unmake();

            if eval > max_eval {
                max_eval = eval;

                if depth == MAX_DEPTH {
                    self.best_move = Some(mv);
                }

                let new_alpha = alpha.max(max_eval);

                if alpha >= beta {
                    break;
                }
            }
        }
    }
}
*/
