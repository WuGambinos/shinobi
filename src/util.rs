use macroquad::color::Color;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::*;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};
use std::fs;
use std::path::PathBuf;

use crate::bitboard::BitBoard;
use crate::Castling;
use crate::IntoEnumIterator;
use crate::Piece;
use crate::Position;
use crate::Side;
use crate::SquareLabel;
use crate::State;
use crate::DARK;
use crate::LIGHT;
use crate::NUM_SQUARES;
use crate::SQUARE_SIZE;
use crate::{draw_rectangle, B_IMG_POS, W_IMG_POS};

pub fn handle_movement(
    old_position: &mut Position,
    position: &mut Position,
    selected_piece: &mut Option<Piece>,
    from_square: &mut Option<SquareLabel>,
    target_square: SquareLabel,
    turn: Side,
) {
    println!("MAIN BITBOARD");
    println!();
    position.main_bitboard.print();

    println!();
    println!("{:?} BITBOARD", turn);
    println!();
    position.side_bitboards[turn as usize].print();

    println!();
    println!("{:?} PIECE BITBOARD", turn);
    println!();

    println!("SELECTED PIECE: {:?}", selected_piece);

    let bit = old_position.side_bitboards[turn as usize].get_bit(target_square as u64);
    if from_square.unwrap() != target_square && bit == 0 {
        position.piece_bitboards[turn as usize][selected_piece.unwrap() as usize]
            .clear_bit(from_square.unwrap());
    } else {
        position.set_bit_on_piece_bitboard(selected_piece.unwrap(), turn, from_square.unwrap());
    }
    position.piece_bitboards[turn as usize][selected_piece.unwrap() as usize].print();
}

pub fn drag_and_drop(
    position: &mut Position,
    from_square: &mut Option<SquareLabel>,
    selected_piece: &mut Option<Piece>,
    pieces_textures: &[Texture2D],
    draw_param: &DrawTextureParams,
) {
    if is_mouse_button_pressed(MouseButton::Left) {
        *from_square = Some(get_square_from_mouse_position(mouse_position()));
        let boards = position.piece_bitboards[position.state.turn as usize];

        *selected_piece = None;

        for piece in Piece::iter() {
            let res = boards[piece as usize].get_bit(from_square.unwrap() as u64);
            if res != 0 {
                *selected_piece = Some(piece);
            }
        }

        if let Some(selected_p) = selected_piece {
            position.piece_bitboards[position.state.turn as usize][*selected_p as usize]
                .clear_bit(from_square.unwrap());
        }
    } else if is_mouse_button_down(MouseButton::Left) {
        piece_follow_mouse(&position, *selected_piece, pieces_textures, draw_param);
    } else if is_mouse_button_released(MouseButton::Left) {
        let target_square: SquareLabel = get_square_from_mouse_position(mouse_position());

        if selected_piece.is_some() {
            println!(
                "PIECE: {:?} FROM: {:?} TARGET: {:?}",
                selected_piece,
                from_square.unwrap(),
                target_square
            );
            let old_turn: Side = position.state.turn;
            let mut old_position: Position = position.clone();
            position.make_move(selected_piece.unwrap(), from_square.unwrap(), target_square);

            handle_movement(
                &mut old_position,
                position,
                selected_piece,
                from_square,
                target_square,
                old_turn,
            );
        }
    }
}

pub fn piece_follow_mouse(
    position: &Position,
    piece: Option<Piece>,
    pieces: &[Texture2D],
    draw_param: &DrawTextureParams,
) {
    if let Some(p) = piece {
        let piece_offset: usize = match position.state.turn {
            Side::White => W_IMG_POS,
            Side::Black => B_IMG_POS,
        };

        let piece_index: usize = match p {
            Piece::Pawn => piece_offset + 3,
            Piece::Bishop => piece_offset,
            Piece::Knight => piece_offset + 2,
            Piece::Rook => piece_offset + 5,
            Piece::Queen => piece_offset + 4,
            Piece::King => piece_offset + 1,
        };

        draw_texture_ex(
            pieces[piece_index],
            mouse_position().0 - SQUARE_SIZE / 2.,
            mouse_position().1 - SQUARE_SIZE / 2.,
            Color::new(1.0, 1.0, 1.0, 1.0),
            draw_param.clone(),
        );
    }
}
pub fn get_square_from_mouse_position(pos: (f32, f32)) -> SquareLabel {
    let x = ((pos.0) / SQUARE_SIZE) as i32;
    let y = ((pos.1 / SQUARE_SIZE) as i32 - 7).abs();

    let square = ((8 * y) + x) as u64;
    SquareLabel::from(square)
}

pub fn draw_white_pieces(position: Position, pieces: &[Texture2D], draw_param: &DrawTextureParams) {
    let white_bitboards = position.piece_bitboards[Side::White as usize];
    let mut j = 0;

    for piece in Piece::iter() {
        let piece_offset = W_IMG_POS;

        let piece_index: usize = match piece {
            Piece::Pawn => piece_offset + 3,
            Piece::Bishop => piece_offset,
            Piece::Knight => piece_offset + 2,
            Piece::Rook => piece_offset + 5,
            Piece::Queen => piece_offset + 4,
            Piece::King => piece_offset + 1,
        };

        while j < 64 {
            let rank = j / 8;
            let file = j % 8;

            let x = (file as f32) * SQUARE_SIZE;
            let y = (7 - rank) as f32 * SQUARE_SIZE;

            let pos = ((white_bitboards[piece as usize]) >> j) & BitBoard(1);

            if pos.0 == 1 {
                draw_texture_ex(
                    pieces[piece_index],
                    x,
                    y,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                    draw_param.clone(),
                );
            }
            j += 1;
        }
        j = 0;
    }
}

pub fn draw_black_pieces(position: Position, pieces: &[Texture2D], draw_param: &DrawTextureParams) {
    let black_bitboards = position.piece_bitboards[Side::Black as usize];
    let mut j = 0;
    for piece in Piece::iter() {
        let piece_offset = B_IMG_POS;

        let piece_index: usize = match piece {
            Piece::Pawn => piece_offset + 3,
            Piece::Bishop => piece_offset,
            Piece::Knight => piece_offset + 2,
            Piece::Rook => piece_offset + 5,
            Piece::Queen => piece_offset + 4,
            Piece::King => piece_offset + 1,
        };

        while j < 64 {
            let rank = j / 8;
            let file = j % 8;

            let x = (file as f32) * SQUARE_SIZE;
            let y = (7 - rank) as f32 * SQUARE_SIZE;

            let pos = ((black_bitboards[piece as usize]) >> j) & BitBoard(1);

            if pos.0 == 1 {
                draw_texture_ex(
                    pieces[piece_index],
                    x,
                    y,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                    draw_param.clone(),
                );
            }
            j += 1;
        }
        j = 0;
    }
}

pub fn draw_pieces(position: Position, pieces: &[Texture2D], draw_param: &DrawTextureParams) {
    draw_white_pieces(position.clone(), pieces, draw_param);
    draw_black_pieces(position, pieces, draw_param)
}

pub fn draw_squares() {
    for i in 0..NUM_SQUARES {
        let rank = i / 8;
        let file = i % 8;
        let color = (rank + file) % 2;

        if color == 0 {
            draw_rectangle(
                (rank as f32) * SQUARE_SIZE,
                (file as f32) * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                LIGHT,
            );
        } else {
            draw_rectangle(
                (rank as f32) * SQUARE_SIZE,
                (file as f32) * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                DARK,
            );
        }
    }
}

pub async fn get_images() -> Vec<Texture2D> {
    let mut images: Vec<Texture2D> = Vec::new();
    let mut image_names: Vec<PathBuf> = fs::read_dir("./chess_assets")
        .unwrap()
        .map(|res| res.unwrap())
        .map(|de| de.path())
        .collect();

    image_names.sort();

    for file in image_names {
        let image: Texture2D = load_texture(file.to_str().unwrap()).await.unwrap();
        images.push(image);
        println!("{}", file.display());
    }

    images
}

pub fn print_board(position: [char; 64]) {
    for i in 0..8 {
        for j in 0..8 {
            let idx = i * 8 + j;
            print!("{} ", position[idx]);
        }
        println!();
    }
}

pub fn load_fen(fen: &str, state: &mut State) -> [char; 64] {
    let mut file = 0;
    let mut rank = 7;

    let fen_board: Vec<&str> = fen.split(' ').collect();
    let main_string: &str = fen_board[0];
    let castle_rights = fen_board[2];

    let split_main: Vec<&str> = main_string.split('/').collect();

    let mut res: [char; 64] = ['.'; 64];

    for s in split_main {
        for c in s.chars() {
            if c.is_ascii_digit() {
                file += (c as u32) - '0' as u32;
            } else {
                let idx = rank * 8 + file as usize;
                res[idx] = c;
                file += 1;
            }
        }

        file = 0;
        rank = rank.wrapping_sub(1);
    }

    for c in castle_rights.chars() {
        if c == 'Q' {
            state.castling_rights.0 |= Castling::WHITE_QUEEN_SIDE;
        } else if c == 'K' {
            state.castling_rights.0 |= Castling::WHITE_KING_SIDE;
        } else if c == 'q' {
            state.castling_rights.0 |= Castling::BLACK_QUEEN_SIDE;
        } else if c == 'k' {
            state.castling_rights.0 |= Castling::BLACK_KING_SIDE;
        }
    }
    res
}
