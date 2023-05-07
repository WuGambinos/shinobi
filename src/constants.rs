use sdl2::*;

use crate::BitBoard;

pub const NUM_SQUARES: u8 = 64;
pub const SQUARE_SIZE: i32 = 60;
pub const SCALE: i32 = 1;
pub const RADIUS: i32 = 5;
pub const DARK: pixels::Color = pixels::Color::RGB(181, 136, 99);
pub const LIGHT: pixels::Color = pixels::Color::RGB(240, 217, 181);

// Hexadecimal Constants
pub const A_FILE: BitBoard = BitBoard(0x0101_0101_0101_0101);
pub const B_FILE: BitBoard = BitBoard(0x0202_0202_0202_0202);
pub const C_FILE: BitBoard = BitBoard(0x0404_0404_0404_0404);
pub const D_FILE: BitBoard = BitBoard(0x0808_0808_0808_0808);
pub const E_FILE: BitBoard = BitBoard(0x1010_1010_1010_1010);
pub const F_FILE: BitBoard = BitBoard(0x2020_2020_2020_2020);
pub const G_FILE: BitBoard = BitBoard(0x4040_4040_4040_4040);
pub const H_FILE: BitBoard = BitBoard(0x8080_8080_8080_8080);

#[rustfmt::skip]
pub const FIRST_RANK    : BitBoard = BitBoard(0x0000_0000_0000_00FF);
#[rustfmt::skip]
pub const SECOND_RANK   : BitBoard = BitBoard(0x0000_0000_0000_FF00);
#[rustfmt::skip]
pub const THIRD_RANK    : BitBoard = BitBoard(0x0000_0000_00FF_0000);
#[rustfmt::skip]
pub const FOURTH_RANK   : BitBoard = BitBoard(0x0000_0000_FF00_0000);
#[rustfmt::skip]
pub const FIFTH_RANK    : BitBoard = BitBoard(0x0000_00FF_0000_0000);
#[rustfmt::skip]
pub const SIXTH_RANK    : BitBoard = BitBoard(0x0000_FF00_0000_0000);
#[rustfmt::skip]
pub const SEVENTH_RANK  : BitBoard = BitBoard(0x00FF_0000_0000_0000);
#[rustfmt::skip]
pub const EIGTH_RANK    : BitBoard = BitBoard(0xFF00_0000_0000_0000);

#[rustfmt::skip]
pub const A1_TO_H8_DIAGONAL : BitBoard = BitBoard(0x8040_2010_0804_0201);
#[rustfmt::skip]
pub const H1_TO_A8_DIAGONAL : BitBoard = BitBoard(0x0102_0408_1020_4080);
#[rustfmt::skip]
pub const LIGHT_SQUARES     : BitBoard = BitBoard(0x55AA_55AA_55AA_55AA);
#[rustfmt::skip]
pub const DARK_SQUARES      : BitBoard = BitBoard(0xAA55_AA55_AA55_AA55);

pub const EMPTY_BITBOARD: BitBoard = BitBoard(0);

pub const B_IMG_POS: usize = 0;
pub const W_IMG_POS: usize = 6;

pub const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
pub const TEST_POS2: &str = "8/8/3p4/1Pp4r/K4p1k/8/4P1P1/1R6 b - - 0 1";
pub const CHECK_POS: &str = "4k3/8/6n1/3Q1/8/8/8/4K3 w - - 0 1";
pub const CHECK_POS2: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
pub const CHECK_POS3: &str = "8/2p5/3p4/KP5r/1R2Pp1k/8/6P1/8 b - - 0 1";
pub const PIN_POS: &str = "4k3/8/8/4r3/8/4Q3/8/3K4 w - - 0 1";
