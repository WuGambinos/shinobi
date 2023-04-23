use sdl2::*;

use crate::BitBoard;

pub const NUM_SQUARES: u8 = 64;
pub const SQUARE_SIZE: i32 = 60;
pub const SCALE: i32 = 1;
pub const RADIUS: i32 = 5;
pub const DARK: pixels::Color = pixels::Color::RGB(181, 136, 99);
pub const LIGHT: pixels::Color = pixels::Color::RGB(240, 217, 181);

// Hexadecimal Constants
pub const A_FILE: u64 = 0x0101_0101_0101_0101;
pub const B_FILE: u64 = 0x0202_0202_0202_0202;
pub const C_FILE: u64 = 0x0404_0404_0404_0404;
pub const D_FILE: u64 = 0x0808_0808_0808_0808;
pub const E_FILE: u64 = 0x1010_1010_1010_1010;
pub const F_FILE: u64 = 0x2020_2020_2020_2020;
pub const G_FILE: u64 = 0x4040_4040_4040_4040;
pub const H_FILE: u64 = 0x8080_8080_8080_8080;

#[rustfmt::skip]
pub const FIRST_RANK    : u64 = 0x0000_0000_0000_00FF;
#[rustfmt::skip]
pub const SECOND_RANK   : u64 = 0x0000_0000_0000_FF00;
#[rustfmt::skip]
pub const THIRD_RANK    : u64 = 0x0000_0000_00FF_0000;
#[rustfmt::skip]
pub const FOURTH_RANK   : u64 = 0x0000_0000_FF00_0000;
#[rustfmt::skip]
pub const FIFTH_RANK    : u64 = 0x0000_00FF_0000_0000;
#[rustfmt::skip]
pub const SIXTH_RANK    : u64 = 0x0000_FF00_0000_0000;
#[rustfmt::skip]
pub const SEVENTH_RANK  : u64 = 0x00FF_0000_0000_0000;
#[rustfmt::skip]
pub const EIGTH_RANK    : u64 = 0xFF00_0000_0000_0000;

#[rustfmt::skip]
pub const A1_TO_H8_DIAGONAL : u64 = 0x8040_2010_0804_0201;
#[rustfmt::skip]
pub const H1_TO_A8_DIAGONAL : u64 = 0x0102_0408_1020_4080;
#[rustfmt::skip]
pub const LIGHT_SQUARES     : u64 = 0x55AA_55AA_55AA_55AA;
#[rustfmt::skip]
pub const DARK_SQUARES      : u64 = 0xAA55_AA55_AA55_AA55;

pub const EMPTY_BITBOARD: BitBoard = BitBoard(0);

pub const B_IMG_POS: usize = 0;
pub const W_IMG_POS: usize = 6;
