use crate::util::*;
use core::fmt;
use std::time::{Duration, Instant};

use inquire::Text;
use inquire::{error::InquireResult, Select};
use log::*;
use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Keycode,
    mouse::MouseState,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
    EventPump,
};
use shinobi_core::perft::perft;
use shinobi_core::{Bot, Engine, Piece, Position, Search, Side, Square, START_POS};

#[derive(Debug)]
enum Mode {
    Perft,
    HumanVHuman,
    CpuVCpu,
    HumanVCpu,
}

pub struct Sdl2State {
    event_pump: EventPump,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    mouse_state: MouseState,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Perft => write!(f, "Perft"),
            Mode::HumanVHuman => write!(f, "Human Vs Human"),
            Mode::CpuVCpu => write!(f, "Cpu vs Cpu"),
            Mode::HumanVCpu => write!(f, "Human vs CPU"),
        }
    }
}

pub fn start() -> InquireResult<()> {
    /* MENU */
    let first_menu = vec!["Load Fen"];
    let _ = Select::new("Select an Option", first_menu).prompt()?;

    let fen = Text::new("Enter FEN")
        .with_default(START_POS)
        .prompt_skippable()?;
    let position = Position::from_fen(&fen.unwrap());
    let mut shinobi = Engine::new(position);

    let second_menu = vec![
        Mode::Perft,
        Mode::CpuVCpu,
        Mode::HumanVCpu,
        Mode::HumanVHuman,
    ];
    let second_ans = Select::new("Select an Option", second_menu).prompt()?;

    /* ENGINE */

    match second_ans {
        Mode::Perft => {
            let depth = Text::new("Enter Depth").with_default("1").prompt()?;
            let start = Instant::now();
            let res = perft(
                &mut shinobi.position,
                &mut shinobi.move_gen,
                depth.parse::<u32>().unwrap(),
            );
            let end = start.elapsed();
            println!(
                "DEPTH: {} NODES: {} TIME: {} ms",
                depth,
                res,
                end.as_millis()
            );
            println!("NPS: {}", (res as f64 / (end.as_secs_f64())) as u64);
        }
        _ => {
            /* VIDEO SETUP */
            let sdl_context = sdl2::init().expect("SDL2 INIT Error");
            let video_subsystem = sdl_context.video().expect("SDL2 Subsystem Error");
            let _image_context = sdl2::image::init(InitFlag::PNG).expect("SDL2 IMAGE Error");

            let window = video_subsystem
                .window("SDL2", 480, 480)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())
                .expect("SDL2 Window Error");

            let mut canvas = window
                .into_canvas()
                .software()
                .build()
                .map_err(|e| e.to_string())
                .expect("SDL2 Canvas Error");

            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

            let mut sdl_state = Sdl2State {
                event_pump: sdl_context.event_pump().expect("SDL2 Event Pump Error"),
                canvas,
                texture_creator,
                mouse_state: MouseState::from_sdl_state(0),
            };

            run_loop(&mut shinobi, &mut sdl_state);
        }
    }

    Ok(())
}

pub fn run_loop(shinobi: &mut Engine, sdl_state: &mut Sdl2State) {
    /* IMAGE STUFF */
    let images = get_images();
    let mut piece: Option<Piece> = None;
    let mut from_square: Option<Square> = None;
    let mut moves = Vec::new();
    let mut bot = Search::new();

    'running: loop {
        if shinobi.position.checkmate(&shinobi.move_gen) {
            println!("CHECKMATE!!!");
            println!("{}", shinobi.position);
            std::process::exit(0);
        }
        if shinobi.position.is_draw(&shinobi.move_gen) {
            println!("DRAW...");
            println!("{}", shinobi.position);
            std::process::exit(0);
        }

        bot.search_position(&mut shinobi.position, &shinobi.move_gen, 5);
        if let Some(best_move) = bot.best_move {
            shinobi.position.make_move(best_move);
        }

        for event in sdl_state.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    println!("{}", shinobi.position);
                    let key = shinobi.zobrist.generate_hash(&shinobi.position);
                    println!("PIECE COUNT");
                    shinobi.position.print_piece_count();
                    println!();
                    println!("EXPETED ZOBRIST KEY {:#X}", key);
                    println!(
                        "ACTUAL ZOBRIST KEY: {:#X}",
                        shinobi.position.state.zobrist_hash
                    );
                    println!();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    debug!("MAIN BITBOARD");
                    shinobi.position.main_bitboard.print();

                    debug!("WHITE BITBOARD");
                    shinobi.position.print_white_bitboard();

                    debug!("ROOK BITBOARD");
                    shinobi
                        .position
                        .piece_bitboard(Piece::Rook, Side::White)
                        .print();

                    debug!("BLACK BITBOARD");
                    shinobi.position.print_black_bitboard();

                    debug!("ROOK BITBOARD");
                    shinobi
                        .position
                        .piece_bitboard(Piece::Rook, Side::Black)
                        .print();

                    debug!("BISHOP BOARD");
                    shinobi
                        .position
                        .piece_bitboard(Piece::Bishop, Side::Black)
                        .print();

                    debug!("PIECES");
                    println!("{}", shinobi.position);

                    debug!("CASTLING: {:?}", shinobi.position.state.castling_rights);
                }
                _ => {}
            }
        }

        draw_squares(&mut sdl_state.canvas).expect("Draw Squares Error");
        draw_pieces(
            &mut sdl_state.canvas,
            &sdl_state.texture_creator,
            &images,
            &shinobi.position,
        )
        .expect("Draw Pieces Error");
        drag_and_drop(
            &mut sdl_state.canvas,
            &sdl_state.texture_creator,
            &images,
            &mut moves,
            &sdl_state.event_pump,
            &mut sdl_state.mouse_state,
            &mut shinobi.position,
            &mut shinobi.move_gen,
            &mut from_square,
            &mut piece,
        )
        .expect("Drag and Drop Error");

        sdl_state.canvas.present();
        std::thread::sleep(Duration::from_millis(250));
    }
}
