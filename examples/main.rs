use shinobi::enums::*;
use shinobi::util::{draw_squares, get_images, load_fen, print_board};
use shinobi::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 480,
        window_height: 480,
        ..Default::default()
    }
}

//#[macroquad::main(window_conf)]
fn main() {
    /*
    let piece_textures: Vec<Texture2D> = get_images().await;
    let draw_param: DrawTextureParams = DrawTextureParams {
        dest_size: Some(vec2(
            piece_textures[0].width() * SCALE,
            piece_textures[0].height() * SCALE,
        )),
        source: None,
        rotation: 0.,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };
    */

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SDL2", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    'running: loop {

    }



    let mut piece: Option<Piece> = None;
    let mut from_square: Option<SquareLabel> = None;
    let mut position = Position::new();
    let castling_rights = position.state.castling_rights;

    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /*
    let test_pos = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let knights_only = "1n4n1/8/8/8/8/8/8/1N4N1 w - - 0 1";
    */

    let grid = load_fen(start_pos, &mut position.state);
    position.from_grid(grid);

    /*
    init_slider_attacks(&mut position, true);
    init_slider_attacks(&mut position, false);

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

    /*
    loop {
        draw_squares();
        draw_pieces(position.clone(), &piece_textures, &draw_param);
        drag_and_drop(
            &mut position,
            &mut from_square,
            &mut piece,
            &piece_textures,
            &draw_param,
        );
        next_frame().await;
    }
    */
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
