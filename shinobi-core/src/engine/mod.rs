pub mod bot;
pub mod tt;
pub mod zobrist;

use strum::IntoEnumIterator;

use crate::mov::Move;
use crate::mov::MoveType;
use crate::MoveGenerator;
use crate::Piece;
use crate::Position;
use crate::Side;
use crate::Zobrist;
use crate::EMPTY_BITBOARD;
use crate::START_POS;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
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

        self.search.searching.store(true, Ordering::Relaxed);
    }

    fn handle_stop(&mut self) {
        self.search.searching.store(false, Ordering::Relaxed);
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

const WEIGHTS: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const LARGE_NUM: i32 = 99_999_999;
const MAX_DEPTH: i32 = 4;
pub static mut BEST_MOVE: Option<Move> = None;

// PIECE SQUARE TABLES

#[rustfmt::skip]
const PAWNS_SQ: [[i32; 8]; 8] = [
    [ 0,  0,  0,    0,   0,   0,  0,  0],
    [50, 50,  50,  50,  50,  50, 50, 50],
    [10, 10,  20,  30,  30,  20, 10, 10],
    [ 5,  5,  10,  25,  25,  10,  5,  5],
    [ 0,  0,   0,  20,  20,   0,  0,  0],
    [ 5, -5, -10,   0,   0, -10, -5,  5],
    [ 5, 10,  10, -20, -20,  10, 10,  5],
    [ 0,  0,   0,   0,   0,   0,  0,  0],
];

#[rustfmt::skip]
const KNIGHT_SQ: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20,   0,   0,   0,   0, -20, -40],
    [-30,   0,  10,  15,  15,  10,   0, -30],
    [-30,   5,  15,  20,  20,  15,   5, -30],
    [-30,   0,  15,  20,  20,  15,   0, -30],
    [-30,   5,  10,  15,  15,  10,   5, -30],
    [-40, -20,   0,   5,   5,   0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

#[rustfmt::skip]
const BISHOP_SQ: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10,   0,   0,   0,   0,   0,   0, -10],
    [-10,   0,   5,  10,  10,   5,   0, -10],
    [-10,   5,   5,  10,  10,   5,   5, -10],
    [-10,   0,  10,  10,  10,  10,   0, -10],
    [-10,  10,  10,  10,  10,  10,  10, -10],
    [-10,   5,   0,   0,   0,   0,   5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

#[rustfmt::skip]
const ROOK_SQ: [[i32; 8]; 8] = [
    [ 0,  0,  0,  0,  0,  0,  0,  0],
    [ 5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [ 0,  0,  0,  5,  5,  0,  0,  0],
];

#[rustfmt::skip]
const QUEEN_SQ: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10,   0,   0,  0,  0,   0,   0, -10],
    [-10,   0,   5,  5,  5,   5,   0, -10],
    [ -5,   0,   5,  5,  5,   5,   0,  -5],
    [  0,   0,   5,  5,  5,   5,   0,  -5],
    [-10,   5,   5,  5,  5,   5,   0, -10],
    [-10,   0,   5,  0,  0,   0,   0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

// MVV_VLA[victim][attacker]
const MVV_LVA: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];

#[derive(Clone)]
pub struct Search {
    searching: Arc<AtomicBool>,
    depth: u8,
    nodes: u32,
    best_move: Option<Move>,
}

impl Search {
    fn new() -> Search {
        Search {
            searching: Arc::new(AtomicBool::new(false)),
            depth: 0,
            nodes: 0,
            best_move: None,
        }
    }

    fn search_position(&mut self, position: &mut Position, move_gen: &MoveGenerator) {
        log::info!("SEARCHED STARTED");
        self.negamax_alpha_beta(position, move_gen, -LARGE_NUM, LARGE_NUM, MAX_DEPTH);
        if let Some(best_move) = self.best_move {
            log::info!("BEST_MOVE: {:?} NODES: {}", best_move, self.nodes);
            println!("bestmove {}", best_move);
        }
        log::info!("SEARCH ENDED");
    }

    fn negamax_alpha_beta(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
        depth: i32,
    ) -> i32 {
        if !self.searching.load(Ordering::Relaxed) {
            return 0;
        }

        if position.is_draw(move_gen) {
            return -2000;
        }

        let current_turn = position.state.current_turn();
        let mut moves = move_gen.generate_legal_moves(position, current_turn);

        self.order_moves(position, &mut moves);


        if depth == 0 || moves.len() == 0 {
            if position.checkmate(move_gen) {
                return -9_999_999;
            }

            return self.evalutate(position);
        }

        self.nodes += 1;

        let mut max_eval = -LARGE_NUM;
        for mv in moves {
            position.make_move(mv);
            let eval = -self.negamax_alpha_beta(position, move_gen, -beta, -alpha, depth - 1);
            position.unmake();

            if eval > max_eval {
                max_eval = eval;

                if depth == MAX_DEPTH {
                    log::debug!("MOVE: {:#?}", mv);
                    self.best_move = Some(mv);
                }

                alpha = alpha.max(max_eval);
                if alpha >= beta {
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
            //return MVV_LVA[piece_captured as usize][mv.piece() as usize];
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

        /*
        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut bitboard = position.piece_bitboard(piece, side);

                while bitboard != EMPTY_BITBOARD {
                    let square = bitboard.bitscan_forward_reset();

                    match piece {
                        Piece::Pawn => (),
                        Piece::Bishop => (),
                        Piece::Knight => (),
                        Piece::Rook => (),
                        Piece::Queen => (),
                        Piece::King => (),
                    }
                }
            }
        }
        */

        let material_score = white_score - black_score;
        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        return material_score * side_to_move;
    }
}
