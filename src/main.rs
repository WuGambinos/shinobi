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
    let knights_only = "1n4n1/8/8/8/8/8/8/1N4N1 w - - 0 1";
    let random_fen = "8/3QR2p/2p2Kp1/2B4p/7P/P4n1P/b3p3/1N2k3 w - - 0 1";
    let grid = load_fen(start_pos, &mut position.state);
    position.from_grid(grid);

    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    let castling_rights = position.state.castling_rights;

    init_slider_attacks(&mut position, true);
    init_slider_attacks(&mut position, false);

    /*
    position.piece_bitboards[Side::White as usize][Piece::Pawn as usize].0 = 0xaa5500;
    position.empty_bitboard &= !position.piece_bitboards[Side::White as usize][Piece::Pawn as usize];
    println!("EMPTY SQUARES");
    position.empty_bitboard.print();
    let board = position.white_pawns_able_double_push();
    println!("WHITE PAWNS THAT CAN DOUBLE ");
    board.print();
    */

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
    println!("QUEEN");
    let occupancy: BitBoard = position.main_bitboard;
    occupancy.print();

    let queen_attacks = position.generate_queen_moves(SquareLabel::D7);
    queen_attacks.print();
    */

    /*
    position.generate_moves();
    position.create_move();
    */

    let depth = 2;
    //println!("PERFT: {}", perft(&mut position, depth));
    let mut res = perft_divide(&mut position, depth);
    print_perft_divide(&mut res.1);

    /*
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
            &mut from_square,
            &mut piece,
        )?;

        canvas.present();
    }
    */

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

fn perft(position: &mut Position, depth: u32) -> u32 {
    let mut num_positions: u32 = 0;
    position.generate_moves(position.state.turn);
    let moves = position.create_move(position.state.turn);
    if depth == 1 {
        return moves.len() as u32;
    }

    for mv in moves {
        let old_position: Position = *position;
        position.make_move(mv.piece, mv.from_square, mv.target_square);
        num_positions += perft(position, depth - 1);

        *position = old_position;
    }

    return num_positions;
}

pub fn print_perft_divide(results: &mut Vec<(String, u64)>) {
    results.sort_by(|a, b| b.0.chars().nth(1).cmp(&a.0.chars().nth(1)));
    let mut total = 0;
    for mv in results {
        println!("{}: {}", mv.0, mv.1);
        total += mv.1;
    }
    println!("NODES: {total}");
}

/*
pub fn print_perft_divide(results: &mut Vec<(String, u64)>) {
    results.sort_by(|a, b| b.0.chars().nth(1).cmp(&a.0.chars().nth(1)));
    let mut total = 0;
    for mv in results {
        println!("{}: {}", mv.0, mv.1);
        total += mv.1;
    }
    println!("NODES: {total}");
}

pub fn perft_divide(board: &mut Board, depth: u8) -> (u64, Vec<(String, u64)>) {
    if depth == 0 {
        return (1, vec![]);
    }

    let mut total_nodes = 0;
    let moves = board.generate_legal_moves();
    let mut result = (0, vec![]);

    for mv in moves {
        let mut piece = board.board()[mv.start_square() as usize].unwrap();
        let old_board = *board;
        board.make_move(&mut piece, mv);

        let child_result = perft_divide(board, depth - 1);
        total_nodes += child_result.0;
        result.1.push((mv.to_string(), child_result.0));

        *board = old_board;
    }

    result.0 = total_nodes;
    result
}
*/
pub fn perft_divide(position: &mut Position, depth: u8) -> (u64, Vec<(String, u64)>) {
    if depth == 0 {
        return (1, vec![]);
    }

    let mut total_nodes = 0;
    position.generate_moves(position.state.turn);
    let moves = position.create_move(position.state.turn);
    let mut result = (0, vec![]);

    for mv in moves {
        let old_position: Position = *position;
        position.make_move(mv.piece, mv.from_square, mv.target_square);
        let child_result = perft_divide(position, depth - 1);
        total_nodes += child_result.0;
        result.1.push((mv.to_string(), child_result.0));

        *position = old_position;
    }

    result.0 = total_nodes;
    result
}
