use crate::mov::*;
use js_sys::Array;
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

    pub fn recieve_position(&self) -> Array {
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];

        white_pieces(
            &mut grid,
            self.position.piece_bitboards[Side::White as usize],
        );

        black_pieces(
            &mut grid,
            self.position.piece_bitboards[Side::Black as usize],
        );

        let res = Array::new();
        for r in 0..8 {
            let inner = Array::new();
            for c in 0..8 {
                inner.push(&JsValue::from(grid[r][c].to_string()));
            }

            res.push(&inner);
        }

        return res;
    }

    pub fn reset_position(&mut self) {
        self.position =
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    pub fn moves(&mut self) -> Vec<Move> {
        let res = moves_helper(&mut self.position, &mut self.move_gen);
        return res.list.to_vec();
    }

    pub fn make_move(&mut self, mv: Move) {
        let _ = console_log::init_with_level(Level::Debug);
        self.position.make_move(mv);
        info!("MOVE: {}", mv);
    }

    pub fn start_perft(&mut self, depth: u32) -> u64 {
        let _ = console_log::init_with_level(Level::Debug);
        info!("STARTING PERFT");
        let final_now = instant::Instant::now();
        let res = perft(&mut self.position, &mut self.move_gen, depth);
        let time = final_now.elapsed().as_micros();
        info!("PERFT COMPLETE IN {} USEC", time);
        return res;
    }

    pub fn search(&mut self) -> Option<Move> {
        let _ = console_log::init_with_level(Level::Debug);
        info!("STARTING SEARCH");

        let mut bot = Bot::new();
        let mv = bot.think(&mut self.position, &mut self.move_gen);

        if let Some(best_mv) = mv {
            /*
            info!("BEST MOVE: {:?}", best_mv);
            self.position.make_move(best_mv);
            */
            return Some(best_mv);
        }

        return None;
    }
}

/*
#[wasm_bindgen]
pub fn print_moves(moves: Vec<Move>) {
        let _ = console_log::init_with_level(Level::Debug);
    for mv in moves {
        info!("{}", mv);
    }
}
*/

fn white_pieces(grid: &mut Vec<Vec<char>>, bitboards: [BitBoard; 6]) {
    for piece in Piece::iter() {
        for rank in 0..8 {
            for file in 0..8 {
                let i = (rank * 8) + file;
                let curr_square = ((bitboards[piece as usize]) >> i) & BitBoard(1);

                let c = file;
                let r = 7 - rank;

                if curr_square.0 == 1 {
                    match piece {
                        Piece::Pawn => {
                            grid[r][c] = 'P';
                        }
                        Piece::Bishop => {
                            grid[r][c] = 'B';
                        }
                        Piece::Knight => {
                            grid[r][c] = 'N';
                        }
                        Piece::Rook => {
                            grid[r][c] = 'R';
                        }
                        Piece::Queen => {
                            grid[r][c] = 'Q';
                        }
                        Piece::King => {
                            grid[r][c] = 'K';
                        }
                    }
                }
            }
        }
    }
}

fn black_pieces(grid: &mut Vec<Vec<char>>, bitboards: [BitBoard; 6]) {
    for piece in Piece::iter() {
        for rank in 0..8 {
            for file in 0..8 {
                let i = (rank * 8) + file;
                let curr_square = ((bitboards[piece as usize]) >> i) & BitBoard(1);

                let c = file;
                let r = 7 - rank;

                if curr_square.0 == 1 {
                    match piece {
                        Piece::Pawn => {
                            grid[r][c] = 'p';
                        }
                        Piece::Bishop => {
                            grid[r][c] = 'b';
                        }
                        Piece::Knight => {
                            grid[r][c] = 'n';
                        }
                        Piece::Rook => {
                            grid[r][c] = 'r';
                        }
                        Piece::Queen => {
                            grid[r][c] = 'q';
                        }
                        Piece::King => {
                            grid[r][c] = 'k';
                        }
                    }
                }
            }
        }
    }
}

pub fn moves_helper(position: &mut Position, move_gen: &mut MoveGenerator) -> MoveList {
    let moves =
        move_gen.generate_legal_moves(position, position.state.current_turn(), MoveType::All);
    return moves;
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
