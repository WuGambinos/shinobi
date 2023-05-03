use std::time::Instant;

use perft::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::mouse::MouseState;
use shinobi::enums::*;
use shinobi::util::*;
use shinobi::*;

fn main() -> Result<(), String> {
    /* VIDEO SETUP */
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("SDL2", 480, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    /* IMAGE STUFF */
    let images = get_images();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    /* CHESS STUFF */
    let mut position = Position::new();
    let grid = load_fen(CHECK_POS2, &mut position.state);
    position.from_grid(grid);
    let mut move_gen = position.move_gen;

    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    let castling_rights = position.state.castling_rights;

    /*
    let start = Instant::now();
    let depth = 4;
    let res = perft(&mut position, &mut move_gen, depth);
    let elapsed = start.elapsed();
    println!("PERFT: {} TIME: {} US", res, elapsed.as_micros());
    println!("NPS: {:.0} ", res as f64 / elapsed.as_secs_f64());
    */

    let start = Instant::now();
    let depth = 4;
    let res = perft_test(&mut position.clone(), &mut move_gen, depth);
    let elasped = start.elapsed();
    //println!("PERFT: {} TIME: {} US", res, elasped.as_micros());

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
                    println!("MAIN BITBOARD");
                    position.main_bitboard.print();

                    println!("WHITE KING SQUARE: {:?}", position.white_king_square);

                    let checks = move_gen.attacks_to_king(&position, position.state.turn);
                    println!("ATTACKS ON KING");
                    checks.print();
                }
                _ => {}
            }
        }

        draw_squares(&mut canvas)?;
        draw_pieces(&mut canvas, &texture_creator, &images, &position)?;
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
        )?;

        canvas.present();
    }

    Ok(())
}

fn debug(position: &Position) {
    println!("MAIN BITBOARD");
    position.print_bitboard(position.main_bitboard);

    println!("WHITE BITBOARD");
    println!();
    position.print_white_bitboard();

    let white_pieces = position.piece_bitboards[Side::White as usize];

    for piece in Piece::iter() {
        println!("PIECE: {:?}", piece);
        println!();
        white_pieces[piece as usize].print();
    }
    let black_pieces = position.piece_bitboards[Side::Black as usize];

    println!("BLACK BITBOARD");
    println!();
    position.print_black_bitboard();

    for piece in Piece::iter() {
        println!("PIECE: {:?}", piece);
        println!();
        black_pieces[piece as usize].print();
    }
}
