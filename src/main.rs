use inquire::{error::InquireResult, Select, Text};
use log::*;
use sdl2::mouse::MouseState;
use shinobi::{enums::*, perft::perft, util::*, *};
use std::fmt;

#[derive(Debug)]
enum Mode {
    Perft,
    HumanVHuman,
    CpuVCpu,
    HumanVCpu,
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

fn main() -> InquireResult<()> {
    env_logger::init();

    info!("SHINOBI");

    /* MENU */
    let first_menu = vec!["Load Fen"];
    let _ = Select::new("Select an Option", first_menu).prompt()?;

    let fen = Text::new("Enter FEN")
        .with_default(START_POS)
        .prompt_skippable()?;

    let second_menu = vec![
        Mode::Perft,
        Mode::CpuVCpu,
        Mode::HumanVCpu,
        Mode::HumanVHuman,
    ];
    let second_ans = Select::new("Select an Option", second_menu).prompt()?;

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

    /* IMAGE STUFF */
    let images = get_images();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().expect("SDL2 Event Pump Error");

    /* CHESS STUFF */
    let mut position = Position::new();
    let grid = load_fen(&fen.unwrap(), &mut position.state);
    position.from_grid(grid);
    let mut move_gen = position.move_gen;

    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    match second_ans {
        Mode::Perft => {
            let depth = Text::new("Enter Depth").with_default("1").prompt()?;
            let res = perft(&mut position, &mut move_gen, depth.parse::<u32>().unwrap());
            println!("DEPTH: {} RES: {}", depth, res);
        }
        _ => {
            let mut moves: Vec<Move> = Vec::new();
            let mut state = MouseState::from_sdl_state(0);
            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,

                        Event::KeyDown {
                            keycode: Some(Keycode::R),
                            ..
                        } => {
                            position = Position::new();
                            position.from_grid(grid);
                        }

                        Event::KeyDown {
                            keycode: Some(Keycode::A),
                            ..
                        } => {
                            debug!("MAIN BITBOARD");
                            position.main_bitboard.print();

                            debug!("WHITE BITBOARD");
                            position.print_white_bitboard();

                            debug!("ROOK BITBOARD");
                            position.piece_bitboard(Piece::Rook, Side::White).print();

                            debug!("BLACK BITBOARD");
                            position.print_black_bitboard();

                            debug!("ROOK BITBOARD");
                            position.piece_bitboard(Piece::Rook, Side::Black).print();

                            debug!("BISHOP BOARD");
                            position.piece_bitboard(Piece::Bishop, Side::Black).print();

                            debug!("PIECES");
                            position.print_pieces();

                            debug!("CASTLING: {:?}", position.state.castling_rights);
                        }
                        _ => {}
                    }
                }

                draw_squares(&mut canvas).expect("Draw Squares Error");
                draw_pieces(&mut canvas, &texture_creator, &images, &position)
                    .expect("Draw Pieces Error");
                drag_and_drop(
                    &mut canvas,
                    &texture_creator,
                    &images,
                    &mut moves,
                    &event_pump,
                    &mut state,
                    &mut position,
                    &mut move_gen,
                    &mut from_square,
                    &mut piece,
                )
                .expect("Drag and Drop Error");

                canvas.present();
            }
        }
    }

    Ok(())
}

