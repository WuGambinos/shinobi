use shinobi::enums::*;
use shinobi::util::{draw_squares, get_images, load_fen, print_board};
use shinobi::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 480,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
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

    let mut piece: Option<Pieces> = None;
    let mut from_square: Option<SquareLabels> = None;
    let mut position = Position::new();
    let castling_rights = position.state.castling_rights;

    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let test_pos = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let grid = load_fen(start_pos, &mut position.state);
    position.from_grid(grid);

    let board = Position::generate_knight_moves(&mut position.piece_bitboards[Side::White as usize][Pieces::Knight as usize]);

    board.print();

    /*
    println!("TURN: {:?}", position.state.turn);
    position.make_move(Pieces::Pawn, SquareLabels::D2, SquareLabels::D4);

    println!("TURN: {:?}", position.state.turn);
    position.make_move(Pieces::Pawn, SquareLabels::D7, SquareLabels::D5);

    println!("TURN: {:?}", position.state.turn);
    position.make_move(Pieces::Knight, SquareLabels::B1, SquareLabels::D5);

    println!("MAIN BITBOARD");
    position.print_bitboard(position.main_bitboard);

    println!("WHITE BITBOARD");
    println!();
    position.print_white_bitboard();

    let white_pieces = position.piece_bitboards[Side::White as usize];

    for piece in Pieces::iter() {
        println!("PIECE: {:?}", piece);
        println!();
        white_pieces[piece as usize].print();
    }
    let black_pieces = position.piece_bitboards[Side::Black as usize];

    println!("BLACK BITBOARD");
    println!();
    position.print_black_bitboard();

    for piece in Pieces::iter() {
        println!("PIECE: {:?}", piece);
        println!();
        black_pieces[piece as usize].print();
    }
    */


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
}
