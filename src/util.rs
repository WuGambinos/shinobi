use macroquad::color::Color;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::*;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};
use std::fs;
use std::path::PathBuf;

use crate::bitboard::BitBoard;
use crate::Castling;
use crate::Position;
use crate::Side;
use crate::State;
use crate::DARK;
use crate::LIGHT;
use crate::NUM_SQUARES;
use crate::SQUARE_SIZE;
use crate::{draw_rectangle, B_IMG_POS, W_IMG_POS};

pub fn drag_and_drop(
    position: &mut Position,
    pieces_textures: &[Texture2D],
    draw_param: &DrawTextureParams,
) {
    if is_mouse_button_pressed(MouseButton::Left) {
    } else if is_mouse_button_down(MouseButton::Left) {
    } else if is_mouse_button_released(MouseButton::Left) {
    }
}

/*
pub fn get_square_from_mouse_position(pos: (f32, f32)) -> {
    let x = ((pos.0) / SQUARE_SIZE) as i32;
    let y = ((pos.1 / SQUARE_SIZE) as i32 - 7).abs();

    let square = ((8 * y) + x) as u8;
    (square, self.get_square(square))
}
*/

pub fn draw_white_pieces(position: Position, pieces: &[Texture2D], draw_param: &DrawTextureParams) {
    let white_bitboards = position.bitboard_pieces[Side::White as usize];
    let mut j = 0;

    for (i, board) in white_bitboards.iter().enumerate() {
        let piece_index: usize = match i {
            0 => W_IMG_POS + 3,
            1 => W_IMG_POS,
            2 => W_IMG_POS + 2,
            3 => W_IMG_POS + 5,
            4 => W_IMG_POS + 4,
            5 => W_IMG_POS + 1,
            _ => 1,
        };

        while j < 64 {
            let rank = j / 8;
            let file = j % 8;

            let x = (file as f32) * SQUARE_SIZE;
            let y = (7 - rank) as f32 * SQUARE_SIZE;

            let pos = (*board >> j) & BitBoard(1);

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
    let black_bitboards = position.bitboard_pieces[Side::Black as usize];
    let mut j = 0;

    for (i, board) in black_bitboards.iter().enumerate() {
        let piece_index: usize = match i {
            0 => B_IMG_POS + 3,
            1 => B_IMG_POS,
            2 => B_IMG_POS + 2,
            3 => B_IMG_POS + 5,
            4 => B_IMG_POS + 4,
            5 => B_IMG_POS + 1,
            _ => 1,
        };

        while j < 64 {
            let rank = j / 8;
            let file = j % 8;

            let x = (file as f32) * SQUARE_SIZE;
            let y = (7 - rank) as f32 * SQUARE_SIZE;

            let pos = (*board >> j) & BitBoard(1);

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
