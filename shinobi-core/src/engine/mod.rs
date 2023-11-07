pub mod bot;
pub mod tt;
pub mod zobrist;

use crate::MoveGenerator;
use crate::Position;
use crate::Zobrist;
use crate::MAX_HALF_MOVES;

pub struct Engine {
    pub position: Position,
    pub move_gen: MoveGenerator,
    pub zobrist: Zobrist,
}

impl Engine {
    pub fn new(position: Position) -> Engine {
        Engine {
            position,
            move_gen: MoveGenerator::new(),
            zobrist: Zobrist::new(),
        }
    }

    pub fn checkmate(&mut self) -> bool {
        let turn = self.position.state.turn;
        self.move_gen
            .generate_legal_moves(&mut self.position, turn)
            .is_empty()
    }

    pub fn is_draw(&mut self) -> bool {
        self.draw_by_fifty_moves()
            | self.draw_by_threefold_repetition()
            | self.draw_by_insufficient_material()
    }
    pub fn draw_by_fifty_moves(&self) -> bool {
        self.position.state.half_move_counter >= MAX_HALF_MOVES
    }

    pub fn draw_by_threefold_repetition(&mut self) -> bool {
        let current_pos_key = self.position.state.zobrist_key;
        let prev_states = &self.position.history.prev_states;
        let mut count = 0;

        for state in prev_states {
            if state.zobrist_key == current_pos_key {
                count += 1;
            }

            if count == 3 {
                return true;
            }
        }

        false
    }

    pub fn draw_by_insufficient_material(&self) -> bool {
        false
    }
}
