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
    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_pos = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let knights_only = "1n4n1/8/8/8/8/8/8/1N4N1 w - - 0 1";
    let random_fen = "8/3QR2p/2p2Kp1/2B4p/7P/P4n1P/b3p3/1N2k3 w - - 0 1";
    let grid = load_fen(random_fen, &mut position.state);
    position.from_grid(grid);

    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    let castling_rights = position.state.castling_rights;

    init_slider_attacks(&mut position, true);
    init_slider_attacks(&mut position, false);

    /*
    println!("BISHOP");
    let bishop_occupancy: BitBoard =
        position.piece_bitboards[Side::White as usize][Piece::Bishop as usize];
    bishop_occupancy.print();

    let bishop_attacks = BitBoard(get_bishop_attacks(
        &mut position,
        SquareLabel::D4 as u64,
        bishop_occupancy.0,
    ));
    bishop_attacks.print();

    println!("ROOK");
    let rook_occupancy: BitBoard =
        position.piece_bitboards[Side::White as usize][Piece::Rook as usize];
    rook_occupancy.print();
    let rook_attacks = BitBoard(get_rook_attacks(
        &mut position,
        SquareLabel::D2 as u64,
        rook_occupancy.0,
    ));
    rook_attacks.print();
    */
    println!("QUEEN");
    let occupancy: BitBoard = position.main_bitboard;
    occupancy.print();

    let queen_attacks = position.generate_queen_moves(SquareLabel::D7);
    queen_attacks.print();
    let mut state = MouseState::from_sdl_state(0);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        draw_squares(&mut canvas)?;
        draw_pieces(&mut canvas, &texture_creator, &images, &position)?;
        drag_and_drop(
            &mut canvas,
            &texture_creator,
            &images,
            &event_pump,
            &mut state,
            &mut position,
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
