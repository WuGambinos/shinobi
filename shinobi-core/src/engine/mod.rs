pub mod bot;
pub mod tt;
pub mod zobrist;

use crate::MoveGenerator;
use crate::Position;
use crate::Zobrist;
use crate::MAX_HALF_MOVES;
use crate::START_POS;

pub struct Uci {
    debug: bool,
}

impl Uci {
    fn new() -> Uci {
        Uci { debug: false }
    }

    fn uci(&self) {
        println!("id name Shinobi");
        println!("id author Lajuan");
        println!();

        println!("uciok");
    }

    fn is_ready(&self) {
        println!("readyok");
    }
}

pub struct Engine {
    pub position: Position,
    pub move_gen: MoveGenerator,
    pub zobrist: Zobrist,
    pub uci: Uci,
}

impl Engine {
    pub fn new(position: Position) -> Engine {
        Engine {
            position,
            move_gen: MoveGenerator::new(),
            zobrist: Zobrist::new(),
            uci: Uci::new(),
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

    pub fn uci_loop(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Can not read user input");

            let command = input.trim();
            self.handle_command(command);
        }
    }

    fn handle_command(&mut self, command: &str) {
        if command == "d" {
            println!("{}", self.position);
        } else if command == "uci" {
            self.uci.uci();
        } else if command.starts_with("debug") {
            self.uci.debug = !self.uci.debug;
        } else if command == "isready" {
        } else if command.starts_with("setoption") {
        } else if command.starts_with("position") {
            if command.contains("startpos") {
                let position = Position::from_fen(START_POS);
                self.position = position;
            } else if command.contains("fen") {
            }
        } else {
            println!("Unknown command: '{}'", command);
        }
    }
}
