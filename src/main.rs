use sdl2::mouse::MouseState;
use shinobi::enums::*;
use shinobi::perft::{perft, perft_test};
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

    perft_test(&mut position.clone(), &mut move_gen, 5);

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

                    println!("WHITE BITBOARD");
                    position.print_white_bitboard();

                    println!("ROOK BITBOARD");
                    position.piece_bitboard(Piece::Rook, Side::White).print();

                    println!("BLACK BITBOARD");
                    position.print_black_bitboard();

                    println!("ROOK BITBOARD");
                    position.piece_bitboard(Piece::Rook, Side::Black).print();

                    println!("PIECES");
                    position.print_pieces();

                    println!("CASTLING: {:?}", position.state.castling_rights);
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
