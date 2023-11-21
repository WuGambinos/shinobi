pub mod bot;
pub mod tt;
pub mod zobrist;

use crate::mov::Move;
use crate::mov::MoveType;
use crate::MoveGenerator;
use crate::Position;
use crate::Side;
use crate::Zobrist;
use crate::START_POS;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Copy)]
pub enum PositionToken {
    Moves,
    Fen,
}

pub struct Uci {
    debug: bool,
}

impl Uci {
    fn new() -> Uci {
        Uci { debug: false }
    }
}

pub enum EngineMode {
    Waiting,
    Thinking,
}

pub struct Engine {
    search_thread: Option<JoinHandle<()>>,
    pub position: Position,
    pub move_gen: MoveGenerator,
    pub zobrist: Zobrist,
    pub uci: Uci,
    pub mode: EngineMode,
    pub search: Search,
}

impl Engine {
    pub fn new(position: Position) -> Engine {
        Engine {
            search_thread: None,
            position,
            move_gen: MoveGenerator::new(),
            zobrist: Zobrist::new(),
            uci: Uci::new(),
            mode: EngineMode::Waiting,
            search: Search::new(),
        }
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
            self.handle_uci();
        } else if command.starts_with("debug") {
            self.uci.debug = !self.uci.debug;
        } else if command == "isready" {
            println!("readyok");
        } else if command.starts_with("setoption") {
        } else if command.starts_with("position") {
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            self.handle_position(command_parts);
        } else if command.starts_with("go") {
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            self.handle_go(command_parts);
        } else if command.starts_with("stop") {
            self.handle_stop();
            if let Some(search_th) = self.search_thread.take() {
                search_th.join().expect("Fatal Thread");
            }
        } else if command.starts_with("quit") {
            std::process::exit(0);
        } else {
            println!("Unknown command: '{}'", command);
        }
    }

    fn handle_uci(&self) {
        println!("id name Shinobi");
        println!("id author Lajuan");
        println!();

        println!("uciok");
    }

    fn handle_go(&mut self, parts: Vec<&str>) {
        let arc_mg = Arc::new(self.move_gen);
        let mut pos = self.position.clone();
        let mut search = self.search.clone();
        search.best_move = None;
        self.search_thread = Some(thread::spawn(move || {
            search.search_position(&mut pos, &arc_mg);
        }));
        /*
        for part in parts {
            if part == "ponder" {}

            if part == "searchmoves" {}
        }
        */
    }

    fn handle_stop(&mut self) {
        self.search.searching = false;
        log::info!("STOP TRIGGERED");
    }

    fn handle_position(&mut self, parts: Vec<&str>) {
        let mut fen: String = String::new();
        let mut parse_fen: bool = true;
        let mut moves: Vec<&str> = Vec::new();
        let mut token: Option<PositionToken> = None;

        for part in parts {
            match part {
                "startpos" => parse_fen = false,
                part if part == "fen" && parse_fen => token = Some(PositionToken::Fen),
                "moves" => token = Some(PositionToken::Moves),
                _ => {
                    if let Some(t) = token {
                        match t {
                            PositionToken::Fen => {
                                fen.push_str(part);
                                fen.push_str(" ");
                            }
                            PositionToken::Moves => moves.push(part),
                        }
                    }
                }
            }
        }

        println!("MOVES: {:?}", moves);

        if fen.is_empty() {
            let position = Position::from_fen(START_POS);
            self.position = position;
        } else {
            let position = Position::from_fen(&fen);
            self.position = position;
        }

        for mv in moves {
            let side = self.position.state.current_turn();
            let moves = self.move_gen.generate_legal_moves(&mut self.position, side);
            for gen_move in moves {
                if mv == gen_move.to_string() {
                    self.position.make_move(gen_move);
                }
            }
        }
    }
}

/*
fn search_position_thread(
    position: &mut Position,
    move_gen: Arc<MoveGenerator>,
    search_info: Arc<SearchInfo>,
) {
    search_position(position, &move_gen, &search_info);
}
*/

/*
fn search_position(position: &mut Position, move_gen: &MoveGenerator, search_info: &SearchInfo) {
    log::info!("SEARCHED STARTED");
    negamax_alpha_beta(
        position,
        move_gen,
        search_info,
        -LARGE_NUM,
        LARGE_NUM,
        MAX_DEPTH,
    );
}
*/

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99999999;
const MAX_DEPTH: i32 = 4;
pub static mut BEST_MOVE: Option<Move> = None;

#[derive(Clone, Copy)]
pub struct Search {
    searching: bool,
    depth: u8,
    nodes: u32,
    best_move: Option<Move>,
}

impl Search {
    fn new() -> Search {
        Search {
            searching: true,
            depth: 0,
            nodes: 0,
            best_move: None,
        }
    }

    fn search_position(&mut self, position: &mut Position, move_gen: &MoveGenerator) {
        log::info!("SEARCHED STARTED");
        /*
        for i in 1..=(MAX_DEPTH) {
            self.negamax_alpha_beta(position, move_gen, search_info, -LARGE_NUM, LARGE_NUM, i);
        }
        */
        self.negamax_alpha_beta(position, move_gen, -LARGE_NUM, LARGE_NUM, MAX_DEPTH);
        if let Some(best_move) = self.best_move {
            println!("BEST_MOVE: {:?} NODES: {}", best_move, self.nodes);
        }
        log::info!("SEARCH ENDED");
    }

    fn negamax_alpha_beta(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {
        if position.is_draw(move_gen) {
            return 0;
        }

        let turn = position.state.current_turn();
        let mut moves = move_gen.generate_legal_moves(position, turn);

        //self.order_moves(position, &mut moves);

        if depth == 0 || moves.len() == 0 {
            if position.checkmate(move_gen) {
                return -9999999;
            }

            return self.evalutate(position);
        }

        self.nodes += 1;

        let mut max_eval = -LARGE_NUM;
        for mv in moves {
            if !self.searching {
                log::info!("SEARCHING STOPPED");
                log::info!("best move: {}", mv);
                break;
            }

            position.make_move(mv);
            let eval = -1 * self.negamax_alpha_beta(position, move_gen, -beta, -alpha, depth - 1);
            position.unmake();

            if eval > max_eval {
                max_eval = eval;

                if depth == MAX_DEPTH {
                    log::debug!("MOVE: {:#?}", mv);
                    self.best_move = Some(mv);
                }

                let new_alpha = alpha.max(max_eval);

                if new_alpha >= beta {
                    break;
                }
            }
        }
        return max_eval;
    }

    fn order_moves(&self, position: &Position, moves: &mut Vec<Move>) {
        moves.sort_by(|a, b| {
            self.score_move(position, *b)
                .cmp(&self.score_move(position, *a))
        });
    }

    fn score_move(&self, position: &Position, mv: Move) -> i32 {
        if mv.move_type() == MoveType::Capture {
            let piece_captured = position.pieces[mv.target() as usize].unwrap().1;
            return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
        } else {
            0
        }
    }
    fn evalutate(&self, position: &Position) -> i32 {
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
        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        return material_score * side_to_move;
    }
}
