pub mod constants;
pub mod enums;
pub mod perft;
use crate::{
    bitboard::BitBoard, castling_rights::Castling, Side, Square, State, A_FILE, B_FILE, C_FILE,
    D_FILE, EIGTH_RANK, E_FILE, FIFTH_RANK, FIRST_RANK, FOURTH_RANK, F_FILE, G_FILE, H_FILE,
    SECOND_RANK, SEVENTH_RANK, SIXTH_RANK, SQUARE_SIZE, THIRD_RANK,
};

/**
 * Returns the square that mouse cursor is hovering over when called
 * */
pub fn get_square_from_mouse_position(pos_x: i32, pos_y: i32) -> Square {
    let x = pos_x / SQUARE_SIZE;
    let y = (pos_y / SQUARE_SIZE - 7).abs();

    let square = ((8 * y) + x) as u64;
    Square::from(square)
}

pub fn load_fen(fen: &str, state: &mut State) -> Result<[char; 64], String> {
    let mut file = 0;
    let mut rank = 7;

    let fen_board: Vec<&str> = fen.trim().split(' ').collect();
    let (main_str, turn, castle_rights, en_passant, half_move_counter, full_move_counter) =
        if fen_board.len() == 4 {
            (fen_board[0], fen_board[1], fen_board[2], "-", "0", "0")
        } else if fen_board.len() == 6 {
            (
                fen_board[0],
                fen_board[1],
                fen_board[2],
                fen_board[3],
                fen_board[4],
                fen_board[5],
            )
        } else {
            return Err("INVALID FEN STRING".to_string());
        };

    let split_main: Vec<&str> = main_str.split('/').collect();

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

    state.en_passant = if en_passant == "-" {
        None
    } else {
        let file = en_passant.chars().next().unwrap();
        let rank = en_passant.chars().nth(1).unwrap();

        let file_num = file as u8 - b'a';
        let rank_num = rank.to_digit(10).unwrap() as u8;

        let square = (rank_num - 1) * 8 + file_num;

        Some(Square::from(square as u64))
    };

    state.half_move_counter = half_move_counter.parse::<u8>().unwrap();
    state.full_move_counter = full_move_counter.parse::<u8>().unwrap();
    Ok(grid)
}

pub fn square_name(square: u8) -> String {
    let rank = (square / 8) + 1;
    let file = ((square % 8) + (b'a')) as char;
    format!("{file}{rank}")
}

/**
 * Returns a Bitboard with the files adjacent to square all set
 * */
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

/**
 * Returns a Bitboard with all the squares in the same file as the given square set
 * */
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

/**
 * Returns a Bitboard with all the squares in the same rank as the given square set
 * */
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

pub fn get_time_ms() -> i32 {
    return chrono::Local::now().timestamp_millis() as i32;
}
