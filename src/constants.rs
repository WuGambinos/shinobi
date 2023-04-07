use macroquad::color::Color;

pub const NUM_SQUARES: u8 = 64;
pub const SQUARE_SIZE: f32 = 60.;
pub const SCALE: f32 = 1.;
pub const DARK: Color = Color::new(0.44, 0.50, 0.56, 1.00);
pub const LIGHT: Color = Color::new(0.78, 0.778, 0.78, 1.00);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.00);
pub const RADIUS: f32 = 5.;

// Hexadecimal Constants
pub const A_FILE: u64 = 0x0101_0101_0101_0101;
pub const B_FILE: u64 = 0x0202_0202_0202_0202;
pub const C_FILE: u64 = 0x0404_0404_0404_0404;
pub const D_FILE: u64 = 0x0808_0808_0808_0808;
pub const E_FILE: u64 = 0x1010_1010_1010_1010;
pub const F_FILE: u64 = 0x2020_2020_2020_2020;
pub const G_FILE: u64 = 0x4040_4040_4040_4040;
pub const H_FILE: u64 = 0x8080_8080_8080_8080;

pub const FIRST_RANK: u64 = 0x0000_0000_0000_00FF;
pub const EIGTH_RANK: u64 = 0xFF00_0000_0000_0000;
pub const A1_TO_H8_DIAGONAL: u64 = 0x8040_2010_0804_0201;
pub const H1_TO_A8_DIAGONAL: u64 = 0x0102_0408_1020_4080;
pub const LIGHT_SQUARES: u64 = 0x55AA_55AA_55AA_55AA;
pub const DARK_SQUARES: u64 = 0xAA55_AA55_AA55_AA55;

pub const B_IMG_POS: usize = 0;
pub const W_IMG_POS: usize = 6;
