use crate::mov::*;
use shinobi_core::*;
use std::sync::Mutex;
use std::time::Instant;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClientEngine {
    position: Position,
    move_gen: MoveGenerator,
    search: search::Search,
}

#[wasm_bindgen]
impl ClientEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ClientEngine {
        ClientEngine {
            position: Position::default(),
            move_gen: MoveGenerator::new(),
            search: search::Search::new(),
        }
    }

    /*
    pub fn moves(&mut self) -> MoveList {
        let move_gen = self.move_gen;
        let position = &mut self.position;
        let res =
            move_gen.generate_legal_moves(position, position.state.current_turn(), MoveType::All);
        return res;
    }
    */
}

#[wasm_bindgen]
pub fn new_engine() -> ClientEngine {
    ClientEngine {
        position: Position::default(),
        move_gen: MoveGenerator::new(),
        search: search::Search::new(),
    }
}

pub fn perft(position: &mut Position, move_gen: &mut MoveGenerator, depth: u32) -> u64 {
    let mut num_positions: u64 = 0;
    let moves =
        move_gen.generate_legal_moves(position, position.state.current_turn(), MoveType::All);

    if depth == 1 {
        return moves.len() as u64;
    }

    for i in 0..moves.len() {
        let mv = moves.get(i);
        position.make_move(mv);
        num_positions += perft(position, move_gen, depth - 1);
        position.unmake();
    }

    return num_positions;
}

#[wasm_bindgen]
pub fn get_perft(e: ClientEngine, depth: u32) -> u64 {
    let mut move_gen = e.move_gen;
    let mut position = e.position;

    let res = perft(&mut position, &mut move_gen, depth);
    return res;
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
