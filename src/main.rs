use std::time::Instant;

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
    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_pos = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let check_pos = "4k3/8/6n1/3Q1/8/8/8/4K3 w - - 0 1";
    let grid = load_fen(start_pos, &mut position.state);
    position.from_grid(grid);
    let mut move_gen = MoveGenerator::new();

    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    let castling_rights = position.state.castling_rights;

    /*
    let start = Instant::now();
    let depth = 4;
    let res = perft(&mut position.clone(), &mut move_gen, depth);
    let elasped = start.elapsed();
    println!("PERFT: {} TIME: {} US", res, elasped.as_micros());
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

fn print_magics(is_bishop: bool) {
    let mut offset = 0;
    for square in 0..64 {
        let entry = if is_bishop {
            find_magic(square, true)
        } else {
            find_magic(square, false)
        };

        println!(
            "    MagicEntry {{ mask: 0x{:0>16X}, magic: 0x{:0>16X}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, offset,
        );
        offset += entry.size;
    }

    if is_bishop {
        println!("pub const BISHOP MAP_SIZE: usize = {};", offset);
        println!();
    } else {
        println!("pub const ROOK MAP_SIZE: usize = {};", offset);
        println!();
    }
}
fn draw_moves(canvas: &mut WindowCanvas, attacks: &BitBoard) -> Result<(), String> {
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            let bit = attacks.get_bit(square);
            if bit == 1 {
                canvas.filled_circle(
                    file as i16 * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                    (7 - rank as i16) * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                    5,
                    Color::RED,
                )?;
            }
        }
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

fn perft(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) -> u32 {
    let mut num_positions: u32 = 0;
    let moves = move_generator.generate_moves(position, position.state.turn);

    if depth == 1 {
        return moves.len() as u32;
    }

    for mv in moves {
        position.make_move(mv.piece, mv.from_square, mv.target_square);
        num_positions += perft(position, move_generator, depth - 1);
        position.unmake();
    }

    return num_positions;
}

static mut nodes: u32 = 0;

fn perft_driver(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) {
    let moves = move_generator.generate_moves(position, position.state.turn);
    if depth == 1 {
        unsafe {
            nodes += moves.len() as u32;
        }
        return;
    }

    for mv in moves {
        position.make_move(mv.piece, mv.from_square, mv.target_square);

        perft_driver(position, move_generator, depth - 1);

        position.unmake();
    }
}

fn perft_test(position: &mut Position, move_generator: &mut MoveGenerator, depth: u32) {
    println!(" PERFORMANCE TEST");

    let moves = move_generator.generate_moves(position, position.state.turn);

    for mv in moves {
        position.make_move(mv.piece, mv.from_square, mv.target_square);

        unsafe {
            let cummulative_nodes: u32 = nodes;
            perft_driver(position, move_generator, depth - 1);

            let old_nodes: u32 = nodes - cummulative_nodes;

            position.unmake();

            println!("{}: {}", mv, old_nodes);
        }
    }

    println!("DEPTH: {}", depth);
    unsafe {
        println!("NODES: {}", nodes);
    }
}
