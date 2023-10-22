pub mod tt;
pub mod zobrist;
pub mod bot;

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
            .len()
            == 0
    }

    pub fn is_draw(&mut self) -> bool {
        self.draw_by_fifty_moves()
            | self.draw_by_threefold_repitiion()
            | self.draw_by_insufficient_material()
    }
    pub fn draw_by_fifty_moves(&self) -> bool {
        self.position.state.half_move_counter >= MAX_HALF_MOVES
    }

    pub fn draw_by_threefold_repitiion(&mut self) -> bool {
        let key = self.zobrist.generate_hash_key(&self.position);

        false
    }

    pub fn draw_by_insufficient_material(&self) -> bool {
        false
    }
}
