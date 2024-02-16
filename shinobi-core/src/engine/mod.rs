pub mod bot;
pub mod tt;
pub mod zobrist;

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
use strum::IntoEnumIterator;

const MAX_TIME_MS: i32 = 200;

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
        } else if command == "ucinewgame" {
            self.search = Search::new();
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
        if parts[1] == "depth" {
            log::info!("DEPTH SEEN");
            let d = parts[2].parse();
            match d {
                Ok(depth) => {
                    let arc_mg = Arc::new(self.move_gen);
                    let mut pos = self.position.clone();
                    let mut search = self.search.clone();
                    search.best_move = None;
                    self.search_thread = Some(thread::spawn(move || {
                        search.search_position(&mut pos, &arc_mg, depth);
                    }));
                }
                Err(_) => {
                    let arc_mg = Arc::new(self.move_gen);
                    let mut pos = self.position.clone();
                    let mut search = self.search.clone();
                    search.best_move = None;
                    self.search_thread = Some(thread::spawn(move || {
                        search.search_position(&mut pos, &arc_mg, MAX_DEPTH);
                    }));
                }
            }
        } else {
            log::info!("NO DEPTH SEEN");
            let arc_mg = Arc::new(self.move_gen);
            let mut pos = self.position.clone();
            let mut search = self.search.clone();
            search.best_move = None;
            self.search_thread = Some(thread::spawn(move || {
                search.search_position(&mut pos, &arc_mg, MAX_DEPTH);
            }));
        }

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
const MAX_DEPTH: i32 = 3;
pub static mut BEST_MOVE: Option<Move> = None;

// PIECE SQUARE TABLES

#[rustfmt::skip]
const PAWNS_SQ: [i32; 64] = [
     0,  0,  0,    0,   0,   0,  0,  0,
    50, 50,  50,  50,  50,  50, 50, 50,
    10, 10,  20,  30,  30,  20, 10, 10,
     5,  5,  10,  25,  25,  10,  5,  5,
     0,  0,   0,  20,  20,   0,  0,  0,
     5, -5, -10,   0,   0, -10, -5,  5,
     5, 10,  10, -20, -20,  10, 10,  5,
     0,  0,   0,   0,   0,   0,  0,  0,
];

#[rustfmt::skip]
const KNIGHT_SQ: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const BISHOP_SQ: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const ROOK_SQ: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];

#[rustfmt::skip]
const QUEEN_SQ: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20,
    -10,   0,   0,  0,  0,   0,   0, -10,
    -10,   0,   5,  5,  5,   5,   0, -10,
     -5,   0,   5,  5,  5,   5,   0,  -5,
      0,   0,   5,  5,  5,   5,   0,  -5,
    -10,   5,   5,  5,  5,   5,   0, -10,
    -10,   0,   5,  0,  0,   0,   0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20,
];

#[rustfmt::skip]
const KING_SQ: [i32; 64] = [
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -30, -40, -40, -50, -50, -40, -40, -30,
  -20, -30, -30, -40, -40, -30, -30, -20,
  -10, -20, -20, -20, -20, -20, -20, -10, 
   20,  20,   0,   0,   0,   0,  20,  20,
   20,  30,  10,   0,   0,  10,  30,  20
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
    pub searching: Arc<AtomicBool>,
    pub depth: u8,
    pub ply: u8,
    pub nodes: u32,
    pub best_move: Option<Move>,
}

impl Search {
    pub fn new() -> Search {
        Search {
            searching: Arc::new(AtomicBool::new(false)),
            depth: 0,
            ply: 0,
            nodes: 0,
            best_move: None,
        }
    }

    pub fn search_position(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        _depth: i32,
    ) {
        // Reset
        self.ply = 0;
        self.nodes = 0;
        self.best_move = None;
        log::info!("SEARCHED STARTED");
        let mut d = 1;
        loop {
            let score = self.negamax(position, move_gen, -LARGE_NUM, LARGE_NUM, d);
            if self.searching.load(Ordering::Relaxed) {
                println!("info score cp {} depth {} nodes {}", score, d, self.nodes);
            }
            d += 1;
            if d == 10 {
                break;
            }
        }

        if let Some(best_move) = self.best_move {
            log::info!("BEST_MOVE: {:?} NODES: {}", best_move, self.nodes);

            println!("bestmove {}", best_move);
        }
        log::info!("SEARCH ENDED");
    }

    #[inline(always)]
    pub fn negamax(
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

        if depth == 0 {
            return self.quiescence(position, move_gen, alpha, beta);
        }

        self.nodes += 1;
        let mut best_so_far: Option<Move> = None;
        let old_alpha = alpha;
        let mut moves = move_gen.generate_legal_moves(position, position.state.current_turn());

        self.order_moves(position, &mut moves);
        for mv in moves.iter() {
            self.ply += 1;
            position.make_move(*mv);
            let score = -self.negamax(position, move_gen, -beta, -alpha, depth - 1);
            self.ply -= 1;
            position.unmake();

            // Fail-hard beta cutoff
            if score >= beta {
                // Move is too "good" (fails high)
                // Opponent will avoid this position
                return beta;
            }

            // Better move found
            if score > alpha {
                // PV Move
                alpha = score;

                let root_move = self.ply == 0;
                if root_move {
                    best_so_far = Some(*mv);
                }
            }
        }

        if moves.len() == 0 {
            if position.checkmate(move_gen) {
                return -LARGE_NUM + self.ply as i32;
            } else {
                return -100;
                //return self.evalutate(position);
            }
        }

        if old_alpha != alpha {
            self.best_move = best_so_far;
        }

        // Position not good enough (Fails low)
        return alpha;
    }

    fn quiescence(
        &mut self,
        position: &mut Position,
        move_gen: &MoveGenerator,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        let eval = self.evalutate(position);

        // Fail-hard beta cutoff
        if eval >= beta {
            // Move is too "good" (fails high)
            // Opponent will avoid this position
            return beta;
        }

        // Better Move found is eval > alpha
        // PV Move
        alpha = alpha.max(eval);

        // FIX THIS (Find other way to only get captures)
        let mut captures: Vec<Move> = move_gen
            .generate_legal_moves(position, position.state.current_turn())
            .into_iter()
            .filter(|item| item.move_type() == MoveType::Capture)
            .collect();

        self.order_moves(position, &mut captures);
        for capture in captures {
            self.ply += 1;
            position.make_move(capture);
            let eval = -self.quiescence(position, move_gen, -beta, -alpha);
            self.ply -= 1;
            position.unmake();

            // Fail-hard beta cutoff
            if eval >= beta {
                // Move is too "good" (fails high)
                // Opponent will avoid this position
                return beta;
            }

            // Better Move found is eval > alpha
            // PV Move
            alpha = alpha.max(eval);
        }
        return alpha;
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
            //return WEIGHTS[piece_captured as usize] - WEIGHTS[mv.piece() as usize] / 10;
            return MVV_LVA[piece_captured as usize][mv.piece() as usize];
        } else {
            0
        }
    }
    fn evalutate(&self, position: &Position) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        /*
        let piece_count = position.piece_count;
        for (i, count) in piece_count[Side::White as usize].iter().enumerate() {
            white_score += WEIGHTS[i] * (*count as i32);
        }

        for (i, count) in piece_count[Side::Black as usize].iter().enumerate() {
            black_score += WEIGHTS[i] * (*count as i32);
        }
        */

        for side in Side::iter() {
            for piece in Piece::iter() {
                let mut bitboard = position.piece_bitboard(piece, side);

                while bitboard != EMPTY_BITBOARD {
                    let square = bitboard.bitscan_forward_reset();

                    let index = if side == Side::White {
                        let mut rank = square as usize / 8;
                        rank = 7 - rank;

                        let file = square as usize % 8;

                        rank * 8 + file
                    } else {
                        square as usize
                    };

                    match piece {
                        Piece::Pawn => {
                            if side == Side::White {
                                white_score += PAWNS_SQ[index as usize];
                            } else {
                                black_score += PAWNS_SQ[index as usize];
                            }
                        }
                        Piece::Bishop => {
                            if side == Side::White {
                                white_score += BISHOP_SQ[index as usize];
                            } else {
                                black_score += BISHOP_SQ[index as usize];
                            }
                        }
                        Piece::Knight => {
                            if side == Side::White {
                                white_score += KNIGHT_SQ[index as usize];
                            } else {
                                black_score += KNIGHT_SQ[index as usize];
                            }
                        }
                        Piece::Rook => {
                            if side == Side::White {
                                white_score += ROOK_SQ[index as usize];
                            } else {
                                black_score += ROOK_SQ[index as usize];
                            }
                        }
                        Piece::Queen => {
                            if side == Side::White {
                                white_score += QUEEN_SQ[index as usize];
                            } else {
                                black_score += QUEEN_SQ[index as usize];
                            }
                        }
                        Piece::King => {
                            if side == Side::White {
                                white_score += KING_SQ[index as usize];
                            } else {
                                black_score += KING_SQ[index as usize];
                            }
                        }
                    }
                }
            }
        }

        let material_score = white_score - black_score;
        let side_to_move = if position.state.current_turn() == Side::White {
            1
        } else {
            -1
        };

        return material_score * side_to_move;
    }
}
