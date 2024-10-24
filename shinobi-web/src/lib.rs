use crate::mov::*;
use log::error;
use log::info;
use log::Level;
use shinobi_core::*;
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

    pub fn load_fen(&mut self, fen: &str) -> Result<(), String> {
        let _ = console_log::init_with_level(Level::Debug);
        let position = Position::from_fen(fen);
        match position {
            Ok(position) => {
                self.position = position;
                return Ok(());
            }
            Err(e) => {
                error!("INVALID FEN: {}", e);
                return Err(e);
            }
        }
    }

    pub fn start_perft(&mut self, depth: u32) -> u64 {
        let _ = console_log::init_with_level(Level::Debug);
        info!("STARTING PERFT");
        return perft(&mut self.position, &mut self.move_gen, depth);
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
