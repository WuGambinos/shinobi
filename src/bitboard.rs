use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Shl, Shr,
    Sub,
};
use std::*;

use crate::SquareLabels;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn get_bit(&self, square: u64) -> u64 {
        let res = (*self & BitBoard(1u64 << square)).0;
        if res == 0 {0} else {1}
    }

    pub fn set_bit(&mut self, square: SquareLabels) {
        self.0 |= 1u64 << (square as u64);
    }

    pub fn clear_bit(&mut self, square: SquareLabels) {
        self.0 &= !(1u64 << (square as u64));
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
