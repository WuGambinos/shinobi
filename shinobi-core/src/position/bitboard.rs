use serde::Serialize;

use crate::square::Square;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Shl, Shr,
    Sub,
};
use std::*;

// BitBoard Constants
pub const EMPTY_BITBOARD: BitBoard = BitBoard(0);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn get_bit(&self, square: u64) -> u64 {
        let res = (*self & BitBoard(1u64 << square)).0;
        if res == 0 {
            0
        } else {
            1
        }
    }

    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1u64 << (square as u64);
    }

    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(1u64 << (square as u64));
    }

    /* *
     * Returns Index (Square) of least significant 1 bit
     */
    pub fn bitscan_forward(&self) -> Square {
        assert!(self.0 != 0);
        let square = self.0.trailing_zeros() as u64;
        Square::from(square)
    }

    /* *
     * Returns Index (Square) of least significant 1 bit and then clear bit at that index
     */
    pub fn bitscan_forward_reset(&mut self) -> Square {
        assert!(self.0 != 0);
        let square = self.bitscan_forward();
        self.0 &= self.0 - 1;
        square
    }

    /* *
     * Returns number of set bits
     */
    pub fn pop_count(&mut self) -> u64 {
        let mut count = 0;
        let mut n = self.0;

        while n > 0 {
            count += 1;
            n &= n - 1;
        }
        count
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = self.get_bit(square);
                print!(" {} ", bit);
            }
            println!();
        }

        println!();
        println!(" A  B  C  D  E  F  G  H ");
        println!();
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            Some(self.bitscan_forward_reset())
        }
    }
}

impl Shr<usize> for BitBoard {
    type Output = BitBoard;

    fn shr(self, n: usize) -> BitBoard {
        BitBoard(self.0 >> n)
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(self, n: usize) -> BitBoard {
        BitBoard(self.0 << n)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        self.0 &= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;
    fn bitxor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Add for BitBoard {
    type Output = BitBoard;
    fn add(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_add(rhs.0))
    }
}

impl Sub for BitBoard {
    type Output = BitBoard;
    fn sub(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_sub(rhs.0))
    }
}

impl Mul for BitBoard {
    type Output = BitBoard;
    fn mul(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(rhs.0))
    }
}

impl Div for BitBoard {
    type Output = BitBoard;
    fn div(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_div(rhs.0))
    }
}
