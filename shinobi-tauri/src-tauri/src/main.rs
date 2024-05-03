// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::Instant;

use shinobi_core::{
    mov::Move, mov::MoveType, search::Search, BitBoard, Bot, Engine, IntoEnumIterator,
    MoveGenerator, Piece, Position, Side, Square,
};
use tauri::State;

pub struct ClientEngine {
    position: Position,
    move_gen: MoveGenerator,
    search: Search,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn white_pieces(grid: &mut [[char; 8]; 8], bitboards: [BitBoard; 6]) {
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

fn black_pieces(grid: &mut [[char; 8]; 8], bitboards: [BitBoard; 6]) {
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

#[tauri::command]
fn recieve_position(engine: State<Mutex<ClientEngine>>) -> [[char; 8]; 8] {
    let mut grid: [[char; 8]; 8] = [['.'; 8]; 8];

    white_pieces(
        &mut grid,
        engine.lock().unwrap().position.piece_bitboards[Side::White as usize],
    );
    black_pieces(
        &mut grid,
        engine.lock().unwrap().position.piece_bitboards[Side::Black as usize],
    );

    return grid;
}

#[tauri::command]
fn reset_position(engine: State<Mutex<ClientEngine>>) -> Result<(), String> {
    let position = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    engine.lock().unwrap().position = position;
    Ok(())
}

#[tauri::command]
fn make_move(engine: State<Mutex<ClientEngine>>, mv: Move) -> Move {
    engine.lock().unwrap().position.make_move(mv);
    return mv;
}

#[tauri::command]
fn piece(engine: State<Mutex<ClientEngine>>, p: Piece, from: Square, to: Square) {
    let mv = Move::init(p, from, to, MoveType::Quiet);
    engine.lock().unwrap().position.make_move(mv);
    println!("MOVE: {:?}", mv);
}

fn perft(position: &mut Position, move_gen: &mut MoveGenerator, depth: u32) -> u64 {
    let mut num_positions: u64 = 0;
    let moves = move_gen.generate_legal_moves(position, position.state.current_turn());

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

#[tauri::command]
fn get_perft(engine: State<Mutex<ClientEngine>>, depth: u32) -> u64 {
    let mut e = engine.lock().unwrap();
    let mut move_gen = e.move_gen;
    let mut pos = &mut e.position;

    let start = Instant::now();
    let res = perft(&mut pos, &mut move_gen, depth);
    let end = start.elapsed();
    println!(
        "DEPTH: {} NODE:S {} TIME: {} ms: ",
        depth,
        res,
        end.as_millis()
    );

    return res;
}

/*
fn wrapper_moves(position: &mut Position, move_gen: &MoveGenerator) -> Vec<Move> {
    let moves = move_gen.generate_legal_moves(position, position.state.current_turn());
    return moves;
}

#[tauri::command]
fn moves(engine: State<Mutex<ClientEngine>>) -> Vec<Move> {
    let mut e = engine.lock().unwrap();
    let move_gen = e.move_gen;
    let mut position = &mut e.position;
    let moves = wrapper_moves(&mut position, &move_gen);
    return moves;
}
*/

#[tauri::command]
fn load_fen(engine: State<Mutex<ClientEngine>>, fen: &str) -> Result<(), String> {
    let position = Position::from_fen(fen);

    match position {
        Ok(position) => {
            engine.lock().unwrap().position = position;
            return Ok(());
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
}

#[tauri::command]
fn search(engine: State<Mutex<ClientEngine>>) {
    println!("SEARCH");
    let mut eng = engine.lock().unwrap();
    let move_gen = eng.move_gen;
    let mut position = &mut eng.position;
    let mut bot = Bot::new();
    let mv = bot.think(position, &move_gen);

    if let Some(best_move) = mv {
        println!("BEST MOVE: {:?}", best_move);
        position.make_move(best_move);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let position = Position::default();
    let engine = Mutex::new(ClientEngine {
        position,
        move_gen: MoveGenerator::new(),
        search: Search::new(),
    });

    tauri::Builder::default()
        .manage(engine)
        .invoke_handler(tauri::generate_handler![
            recieve_position,
            make_move,
            reset_position,
            //piece,
            get_perft,
            //moves,
            load_fen,
            search,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
