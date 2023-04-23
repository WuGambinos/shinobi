use std::fs;
use std::path::PathBuf;

use crate::bitboard::BitBoard;
use crate::Castling;
use crate::Color;
use crate::EventPump;
use crate::IntoEnumIterator;
use crate::LoadTexture;
use crate::Move;
use crate::MoveGenerator;
use crate::Piece;
use crate::Position;
use crate::Rect;
use crate::Side;
use crate::SquareLabel;
use crate::State;
use crate::TextureCreator;
use crate::WindowCanvas;
use crate::WindowContext;
use crate::A_FILE;
use crate::B_FILE;
use crate::B_IMG_POS;
use crate::C_FILE;
use crate::DARK;
use crate::D_FILE;
use crate::EIGTH_RANK;
use crate::E_FILE;
use crate::FIFTH_RANK;
use crate::FIRST_RANK;
use crate::FOURTH_RANK;
use crate::F_FILE;
use crate::G_FILE;
use crate::H_FILE;
use crate::LIGHT;
use crate::SECOND_RANK;
use crate::SEVENTH_RANK;
use crate::SIXTH_RANK;
use crate::SQUARE_SIZE;
use crate::THIRD_RANK;
use crate::W_IMG_POS;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::mouse::MouseButton::Left;
use sdl2::mouse::MouseState;

pub fn get_square_from_mouse_position(pos_x: i32, pos_y: i32) -> SquareLabel {
    let x = pos_x / SQUARE_SIZE;
    let y = (pos_y / SQUARE_SIZE - 7).abs();

    let square = ((8 * y) + x) as u64;
    SquareLabel::from(square)
}

pub fn piece_follow_mouse(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &EventPump,
    pieces: &Vec<PathBuf>,
    position: &Position,
    piece: Option<Piece>,
) -> Result<(), String> {
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

        let square: Rect = Rect::new(
            event_pump.mouse_state().x() - SQUARE_SIZE / 2,
            event_pump.mouse_state().y() - SQUARE_SIZE / 2,
            60,
            60,
        );

        let texture = texture_creator.load_texture(pieces[piece_index].clone())?;
        canvas.copy(&texture, None, square)?;
    }
    Ok(())
}

pub fn drag_and_drop(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &Vec<PathBuf>,
    moves: &mut Vec<Move>,
    event_pump: &EventPump,
    old_state: &mut MouseState,
    position: &mut Position,
    move_gen: &mut MoveGenerator,
    from_square: &mut Option<SquareLabel>,
    selected_piece: &mut Option<Piece>,
) -> Result<(), String> {
    // HELD DOWN
    if old_state.is_mouse_button_pressed(Left)
        && event_pump.mouse_state().is_mouse_button_pressed(Left)
    {
        piece_follow_mouse(
            canvas,
            texture_creator,
            event_pump,
            pieces,
            position,
            *selected_piece,
        )?;

        if let Some(p) = *selected_piece {
            // Draw Moves
            for mv in moves {
                if mv.piece == p && mv.from_square == from_square.unwrap() {
                    let file = mv.target_square as i16 % 8;
                    let rank = mv.target_square as i16 / 8;
                    canvas.filled_circle(
                        file as i16 * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                        (7 - rank as i16) * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                        5,
                        Color::RED,
                    )?;
                }
            }
        }
    }
    // Pressed
    else if event_pump.mouse_state().is_mouse_button_pressed(Left) {
        let state = event_pump.mouse_state();

        *from_square = Some(get_square_from_mouse_position(state.x(), state.y()));
        let boards = position.piece_bitboards[position.state.turn as usize];

        *selected_piece = None;

        for piece in Piece::iter() {
            let res = boards[piece as usize].get_bit(from_square.unwrap() as u64);
            if res != 0 {
                *selected_piece = Some(piece);
            }
        }

        *moves = move_gen.generate_moves(&position, position.state.turn);

        if let Some(selected_p) = selected_piece {
            position.piece_bitboards[position.state.turn as usize][*selected_p as usize]
                .clear_bit(from_square.unwrap());
        }

        *old_state = event_pump.mouse_state();
    }
    // Release Button
    else if !event_pump.mouse_state().is_mouse_button_pressed(Left) {
        let target_square: SquareLabel = get_square_from_mouse_position(
            event_pump.mouse_state().x(),
            event_pump.mouse_state().y(),
        );

        if selected_piece.is_some() {
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
        *selected_piece = None;
        *old_state = MouseState::from_sdl_state(0);
    }

    Ok(())
}

pub fn draw_squares(canvas: &mut WindowCanvas) -> Result<(), String> {
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;

        let color = (rank + file) % 2;

        let rect: Rect = Rect::new(
            rank * SQUARE_SIZE,
            file * SQUARE_SIZE,
            SQUARE_SIZE as u32,
            SQUARE_SIZE as u32,
        );
        if color == 0 {
            canvas.set_draw_color(LIGHT);
            canvas.fill_rect(rect)?;
        } else {
            canvas.set_draw_color(DARK);
            canvas.fill_rect(rect)?;
        }
    }
    Ok(())
}

pub fn get_images() -> Vec<PathBuf> {
    let mut image_paths: Vec<PathBuf> = fs::read_dir("./chess_assets")
        .unwrap()
        .map(|res| res.unwrap())
        .map(|de| de.path())
        .collect();

    image_paths.sort();

    image_paths
}

pub fn draw_pieces(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &Vec<PathBuf>,
    position: &Position,
) -> Result<(), String> {
    draw_white_pieces(canvas, texture_creator, pieces, position)?;
    draw_black_pieces(canvas, texture_creator, pieces, position)?;
    Ok(())
}

pub fn draw_white_pieces(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &Vec<PathBuf>,
    position: &Position,
) -> Result<(), String> {
    let black_bitboards = position.piece_bitboards[Side::White as usize];
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

            let x = (file) * (SQUARE_SIZE as i32);
            let y = (7 - rank) * (SQUARE_SIZE as i32);

            let pos = ((black_bitboards[piece as usize]) >> (j as usize)) & BitBoard(1);

            if pos.0 == 1 {
                let square: Rect = Rect::new(x, y, 60, 60);
                let texture = texture_creator.load_texture(pieces[piece_index].clone())?;
                canvas.copy(&texture, None, square)?;
            }
            j += 1;
        }
        j = 0;
    }

    Ok(())
}

pub fn draw_black_pieces(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &Vec<PathBuf>,
    position: &Position,
) -> Result<(), String> {
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

            let x = (file) * (SQUARE_SIZE as i32);
            let y = (7 - rank) * (SQUARE_SIZE as i32);

            let pos = ((black_bitboards[piece as usize]) >> (j as usize)) & BitBoard(1);

            if pos.0 == 1 {
                let square: Rect = Rect::new(x, y, 60, 60);
                let texture = texture_creator.load_texture(pieces[piece_index as usize].clone())?;
                canvas.copy(&texture, None, square)?;
            }
            j += 1;
        }
        j = 0;
    }

    Ok(())
}

pub fn handle_movement(
    old_position: &mut Position,
    position: &mut Position,
    selected_piece: &mut Option<Piece>,
    from_square: &mut Option<SquareLabel>,
    target_square: SquareLabel,
    turn: Side,
) {
    /*
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
    */

    let bit = old_position.side_bitboards[turn as usize].get_bit(target_square as u64);
    if from_square.unwrap() != target_square && bit == 0 {
        position.piece_bitboards[turn as usize][selected_piece.unwrap() as usize]
            .clear_bit(from_square.unwrap());
    } else {
        position.set_bit_on_piece_bitboard(selected_piece.unwrap(), turn, from_square.unwrap());
    }
    //position.piece_bitboards[turn as usize][selected_piece.unwrap() as usize].print();
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

pub fn square_name(square: u8) -> String {
    let rank = (square / 8) + 1;
    let file = ((square % 8) + (b'a')) as char;
    format!("{file}{rank}")
}

pub fn get_file(square: SquareLabel) -> u64 {
    let file = square as u64 % 8;

    match file {
        0 => A_FILE,
        1 => B_FILE,
        2 => C_FILE,
        3 => D_FILE,
        4 => E_FILE,
        5 => F_FILE,
        6 => G_FILE,
        7 => H_FILE,

        _ => panic!("NOT A FILE"),
    }
}

pub fn get_rank(square: SquareLabel) -> u64 {
    let rank = square as u64 / 8;

    match rank {
        0 => FIRST_RANK,
        1 => SECOND_RANK,
        2 => THIRD_RANK,
        3 => FOURTH_RANK,
        4 => FIFTH_RANK,
        5 => SIXTH_RANK,
        6 => SEVENTH_RANK,
        7 => EIGTH_RANK,

        _ => panic!("NOT A FILE"),
    }
}
