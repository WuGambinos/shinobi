use sdl2::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{mouse::MouseButton::Left, mouse::MouseState};
use shinobi_core::IntoEnumIterator;
use shinobi_core::{
    bitboard::BitBoard, castling_rights::Castling, mov::Move, MoveGenerator, Piece, Position, Side,
    Square, State, A_FILE, B_FILE, B_IMG_POS, C_FILE, D_FILE, EIGTH_RANK, E_FILE, FIFTH_RANK,
    FIRST_RANK, FOURTH_RANK, F_FILE, G_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK, SIXTH_RANK,
    SQUARE_SIZE, THIRD_RANK, W_IMG_POS,
};
use std::{fs, path::PathBuf};

pub const DARK: sdl2::pixels::Color = sdl2::pixels::Color::RGB(181, 136, 99);
pub const LIGHT: sdl2::pixels::Color = sdl2::pixels::Color::RGB(240, 217, 181);

pub fn get_square_from_mouse_position(pos_x: i32, pos_y: i32) -> Square {
    let x = pos_x / SQUARE_SIZE;
    let y = (pos_y / SQUARE_SIZE - 7).abs();

    let square = ((8 * y) + x) as u64;
    Square::from(square)
}

pub fn drag_and_drop(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &[PathBuf],
    moves: &mut Vec<Move>,
    event_pump: &EventPump,
    old_state: &mut MouseState,
    position: &mut Position,
    move_gen: &mut MoveGenerator,
    from_square: &mut Option<Square>,
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
            draw_moves(p, from_square.unwrap(), moves, canvas)?;
        }
    }
    // Pressed
    else if event_pump.mouse_state().is_mouse_button_pressed(Left) {
        let state = event_pump.mouse_state();

        *from_square = Some(get_square_from_mouse_position(state.x(), state.y()));
        let boards = position.piece_bitboards[position.state.current_turn() as usize];

        *selected_piece = None;

        for piece in Piece::iter() {
            let res = boards[piece as usize].get_bit(from_square.unwrap() as u64);
            if res != 0 {
                *selected_piece = Some(piece);
            }
        }

        let turn = position.state.current_turn();
        *moves = move_gen.generate_legal_moves(position, turn);

        if let Some(selected_p) = selected_piece {
            position.piece_bitboards[position.state.current_turn() as usize][*selected_p as usize]
                .clear_bit(from_square.unwrap());
        }

        *old_state = event_pump.mouse_state();
    }
    // Release Button
    else if !event_pump.mouse_state().is_mouse_button_pressed(Left) {
        let target_square: Square = get_square_from_mouse_position(
            event_pump.mouse_state().x(),
            event_pump.mouse_state().y(),
        );

        if let Some(select_piece) = selected_piece {
            let old_turn: Side = position.state.current_turn();
            let mut old_position: Position = position.clone();
            let mut valid = false;
            for mv in moves.iter() {
                if is_valid_move(mv, select_piece, from_square.unwrap(), target_square) {
                    apply_move(position, mv, &mut old_position, old_turn);
                    valid = true;
                    break;
                }
            }

            // Undo visual move
            if !valid {
                position.piece_bitboards[position.state.current_turn() as usize][*select_piece as usize]
                    .set_bit(from_square.unwrap());
            }
        }
        *selected_piece = None;
        *old_state = MouseState::from_sdl_state(0);
    }

    Ok(())
}

fn draw_moves(
    p: Piece,
    from: Square,
    moves: &Vec<Move>,
    canvas: &mut WindowCanvas,
) -> Result<(), String> {
    for mv in moves {
        if mv.piece == p && mv.from == from {
            let file = mv.target as i16 % 8;
            let rank = mv.target as i16 / 8;
            canvas.filled_circle(
                file * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                (7 - rank) * SQUARE_SIZE as i16 + (SQUARE_SIZE / 2) as i16,
                5,
                sdl2::pixels::Color::RED,
            )?;
        }
    }
    Ok(())
}

fn piece_follow_mouse(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &EventPump,
    pieces: &[PathBuf],
    position: &Position,
    piece: Option<Piece>,
) -> Result<(), String> {
    if let Some(p) = piece {
        let piece_offset: usize = match position.state.current_turn() {
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

fn is_valid_move(
    mv: &Move,
    selected_piece: &Piece,
    from_square: Square,
    target_square: Square,
) -> bool {
    mv.piece() == *selected_piece
        && mv.from() == from_square
        && mv.target() == target_square
}

fn apply_move(position: &mut Position, mv: &Move, old_position: &mut Position, old_turn: Side) {
    position.make_move(*mv);
    handle_movement(
        old_position,
        position,
        &mv.piece,
        mv.from,
        mv.target,
        old_turn,
    );
}

fn handle_movement(
    old_position: &mut Position,
    position: &mut Position,
    selected_piece: &Piece,
    from_square: Square,
    target_square: Square,
    turn: Side,
) {
    let bit = old_position.side_bitboards[turn as usize].get_bit(target_square as u64);
    if from_square != target_square && bit == 0 {
        position.piece_bitboards[turn as usize][*selected_piece as usize].clear_bit(from_square);
    } else {
        position.set_bit_on_piece_bitboard(*selected_piece, turn, from_square);
    }
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
    pieces: &[PathBuf],
    position: &Position,
) -> Result<(), String> {
    draw_white_pieces(canvas, texture_creator, pieces, position)?;
    draw_black_pieces(canvas, texture_creator, pieces, position)?;
    Ok(())
}

fn draw_white_pieces(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &[PathBuf],
    position: &Position,
) -> Result<(), String> {
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

            let x = file * SQUARE_SIZE;
            let y = (7 - rank) * SQUARE_SIZE;

            let pos = ((white_bitboards[piece as usize]) >> (j as usize)) & BitBoard(1);

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

fn draw_black_pieces(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    pieces: &[PathBuf],
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

            let x = file * SQUARE_SIZE;
            let y = (7 - rank) * SQUARE_SIZE;

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

    let fen_board: Vec<&str> = fen.trim().split(' ').collect();
    let (main_string, turn, castle_rights, en_passant_square, half_move_counter, full_move_counter) =
        if fen_board.len() == 4 {
            (fen_board[0], fen_board[1], fen_board[2], "-", "0", "0")
        } else {
            (
                fen_board[0],
                fen_board[1],
                fen_board[2],
                fen_board[3],
                fen_board[4],
                fen_board[5],
            )
        };

    let split_main: Vec<&str> = main_string.split('/').collect();

    let mut grid: [char; 64] = ['.'; 64];

    if turn == "b" {
        state.current_turn = Side::Black;
    } else if turn == "w" {
        state.current_turn = Side::White;
    }

    for s in split_main {
        for c in s.chars() {
            if c.is_ascii_digit() {
                file += (c as u32) - '0' as u32;
            } else {
                let idx = rank * 8 + file as usize;
                grid[idx] = c;
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

    state.en_passant = if en_passant_square == "-" {
        None
    } else {
        let file = en_passant_square.chars().next().unwrap();
        let rank = en_passant_square.chars().nth(1).unwrap();

        let file_num = file as u8 - b'a';
        let rank_num = rank.to_digit(10).unwrap() as u8;

        let square = (rank_num - 1) * 8 + file_num;

        Some(Square::from(square as u64))
    };

    state.half_move_counter = half_move_counter.parse::<u8>().unwrap();
    state.full_move_counter = full_move_counter.parse::<u8>().unwrap();
    grid
}

pub fn square_name(square: u8) -> String {
    let rank = (square / 8) + 1;
    let file = ((square % 8) + (b'a')) as char;
    format!("{file}{rank}")
}

pub fn adjacent_files(square: Square) -> BitBoard {
    let file = square as u64 % 8;

    match file {
        0 => B_FILE,
        1 => A_FILE | C_FILE,
        2 => B_FILE | D_FILE,
        3 => C_FILE | E_FILE,
        4 => D_FILE | F_FILE,
        5 => E_FILE | G_FILE,
        6 => F_FILE | H_FILE,
        7 => G_FILE,
        _ => panic!("NOT A FILE"),
    }
}
pub fn get_file(square: Square) -> BitBoard {
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

pub fn get_rank(square: Square) -> BitBoard {
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
