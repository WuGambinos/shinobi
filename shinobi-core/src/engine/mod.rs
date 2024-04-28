pub mod bot;
pub mod search;
pub mod tt;
pub mod zobrist;
pub mod pv;

use crate::get_time_ms;
use crate::mov::Move;
use crate::MoveGenerator;
use crate::Position;
use crate::Zobrist;
use crate::START_POS;
use search::{Search, MAX_DEPTH};
use serde::{ser::SerializeStruct, Serialize};

use std::iter::Peekable;
use std::slice;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;

const MAX_TIME_MS: i32 = 200;
type PeekStrIter<'a> = Peekable<slice::Iter<'a, &'a str>>;

#[derive(Serialize)]
pub enum EngineMode {
    Waiting,
    Thinking,
}

#[derive(Debug, Clone)]
pub struct SearchInfo {
    search_moves: Vec<Move>,
    ponder: bool,
    w_time: Option<i32>,
    b_time: Option<i32>,
    w_inc: Option<i32>,
    b_inc: Option<i32>,
    moves_to_go: Option<i32>,
    depth: Option<i32>,
    nodes: Option<u32>,
    mate: Option<u32>,
    move_time: Option<i32>,
    infinite: bool,
    start_time: Option<i32>,
    stop_time: Option<i32>,
    stopped: bool,
}

impl SearchInfo {
    fn new() -> SearchInfo {
        SearchInfo {
            search_moves: Vec::new(),
            ponder: false,
            w_time: None,
            b_time: None,
            w_inc: None,
            b_inc: None,
            moves_to_go: None,
            depth: None,
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
            start_time: None,
            stop_time: None,
            stopped: false,
        }
    }

    fn reset(&mut self) {
        *self = SearchInfo::new();
    }
}

pub struct Engine {
    pub position: Position,
    pub move_gen: MoveGenerator,
    pub zobrist: Zobrist,
    pub debug: bool,
    pub mode: EngineMode,
    pub info: SearchInfo,
    pub search: Search,
    search_thread: Option<JoinHandle<()>>,
}

impl Serialize for Engine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Engine", 1)?;
        state.serialize_field("position", &self.position)?;
        state.serialize_field("ENGINE MODE", &self.mode)?;
        state.end()
    }
}

impl Engine {
    pub fn new() -> Engine {
        let position = Position::default();
        Engine {
            position,
            move_gen: MoveGenerator::new(),
            zobrist: Zobrist::new(),
            debug: false,
            mode: EngineMode::Waiting,
            info: SearchInfo::new(),
            search: Search::new(),
            search_thread: None,
        }
    }

    pub fn run(&mut self) {
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
        let mut arguments: Vec<&str> = command.split_whitespace().collect();
        let command = arguments.remove(0);

        match command {
            "d" => println!("{}", self.position),
            "uci" => self.handle_uci(),
            "debug" => self.debug = !self.debug,
            "isready" => println!("readyok"),
            "ucinewgame" => self.search = Search::new(),
            "setoption" => (),
            "position" => match self.handle_position(arguments) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                }
            },
            "go" => {
                self.handle_go(arguments);
            }
            "stop" => {
                self.handle_stop();
                if let Some(search_th) = self.search_thread.take() {
                    search_th.join().expect("Fatal Thread");
                }
            }

            "quit" => std::process::exit(0),
            _ => log::error!("Unknown command: {}", command),
        }
    }

    fn handle_uci(&self) {
        println!("id name Shinobi");
        println!("id author Lajuan");
        println!();

        println!("uciok");
    }

    fn parse_go_commands<T: FromStr>(&mut self, iterator: &mut PeekStrIter, data: &mut Option<T>) {
        if let Some(time) = iterator.peek() {
            match time.parse::<T>() {
                Ok(v) => {
                    *data = Some(v);
                    self.info.infinite = false
                }

                Err(_) => {
                    self.info.infinite = false;
                }
            }
        } else {
            self.info.infinite = true;
        }
    }

    fn handle_go(&mut self, args: Vec<&str>) {
        self.info.reset();
        let mut time = None;
        let mut inc = 0;
        let mut iterator: Peekable<slice::Iter<'_, &str>> = args.iter().peekable();
        while let Some(arg) = iterator.next() {
            match *arg {
                "wtime" => {
                    let mut data = self.info.w_time;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.w_time = data;
                    time = data;
                }
                "btime" => {
                    let mut data = self.info.b_time;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.b_time = data;
                    time = data;
                }
                "winc" => {
                    let mut data = self.info.w_inc;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.w_inc = data;
                    inc = data.unwrap();
                }
                "binc" => {
                    let mut data = self.info.b_inc;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.b_inc = data;
                    inc = data.unwrap();
                }
                "movestogo" => {
                    let mut data = self.info.moves_to_go;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.moves_to_go = data;
                }
                "depth" => {
                    let mut data = self.info.depth;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.depth = data;
                }

                /*
                "nodes" => {
                    let mut data = self.info.nodes;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.nodes = data;
                }

                "mate" => {
                    let mut data = self.info.mate;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.mate = data;
                }

                "movetime" => {
                    let mut data = self.info.move_time;
                    self.parse_go_commands(&mut iterator, &mut data);
                    self.info.move_time = data;
                }

                "ponder" => self.info.ponder = true,

                "infinite" => self.info.infinite = true,
                */
                _ => (),
            }
        }

        /*
        if let Some(_) = self.info.move_time {
            time = self.info.move_time;
            self.info.moves_to_go = Some(1);
        }
        */

        self.info.start_time = Some(get_time_ms());

        if let Some(t) = time {
            self.info.infinite = false;
            time = Some(t / self.info.moves_to_go.unwrap());
            time = Some(t - 50);
            self.info.stop_time = Some(self.info.start_time.unwrap() as i32 + t as i32 + inc);
        }

        log::info!(
            "TIME: {:#?} START: {:?} STOP: {:?} DEPTH: {:?} TIMESET: {:?}",
            time,
            self.info.start_time,
            self.info.stop_time,
            self.info.depth,
            !self.info.infinite
        );

        if let Some(depth) = self.info.depth {
            log::info!("DEPTH SEEN");
            let arc_mg = std::sync::Arc::new(self.move_gen);
            let mut pos = self.position.clone();
            let mut search = self.search.clone();
            search.best_move = None;
            let mut info = self.info.clone();
            self.search_thread = Some(std::thread::spawn(move || {
                search.search_position(&mut info, &mut pos, &arc_mg, depth);
            }));
        } else {
            log::info!("MAX DEPTH");
            let arc_mg = std::sync::Arc::new(self.move_gen);
            let mut pos = self.position.clone();
            let mut search = self.search.clone();
            search.best_move = None;
            let mut info = self.info.clone();
            self.search_thread = Some(std::thread::spawn(move || {
                search.search_position(&mut info, &mut pos, &arc_mg, MAX_DEPTH);
            }));
        }

        self.search.searching.store(true, Ordering::Relaxed);

        /*
        if args[1] == "depth" {
            log::info!("DEPTH SEEN");
            let d = args[2].parse();
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
        */
    }

    fn handle_stop(&mut self) {
        self.search.searching.store(false, Ordering::Relaxed);
        log::info!("STOP TRIGGERED");
    }

    fn handle_position(&mut self, args: Vec<&str>) -> Result<(), String> {
        let mut fen: String = String::new();
        let mut parse_fen: bool = true;
        let mut moves: Vec<&str> = Vec::new();

        if let Some(arg_0) = args.first() {
            match *arg_0 {
                "startpos" => parse_fen = false,
                "fen" => parse_fen = true,
                _ => return Err(format!("Invalid position argument: {}", args[0])),
            }
        } else {
            return Err("MISSING ARGUMENTS".to_string());
        }

        if parse_fen {
            for arg in args[1..].iter() {
                if *arg == "moves" {
                    break;
                }
                fen.push_str(arg);
                fen.push(' ');
            }
        }

        let mut parsing_moves = false;
        for arg in args.iter() {
            if parsing_moves {
                moves.push(arg);
            }

            if *arg == "moves" {
                parsing_moves = true;
            }
        }

        if fen.is_empty() {
            let position = Position::from_fen(START_POS);
            self.position = position.unwrap();
        } else {
            let position = Position::from_fen(&fen);
            match position {
                Ok(position) => self.position = position,
                Err(e) => return Err(e),
            }
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

        Ok(())
    }
}
