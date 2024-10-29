use crate::BitBoard;
use crate::MoveGenerator;
use crate::BISHOP_MAGICS;
use crate::BIT_TABLE;
use crate::EMPTY_BITBOARD;
use crate::ROOK_MAGICS;
use rand::prelude::*;

pub mod magic_constants;

#[derive(Clone, Copy, Debug)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub size: usize,
}

impl MagicEntry {
    pub fn new(mask: u64, magic: u64, shift: u32, size: usize) -> MagicEntry {
        MagicEntry {
            mask,
            magic,
            shift,
            size,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SMagic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub offset: usize,
}

impl SMagic {
    pub fn new(mask: u64, magic: u64, shift: u32, offset: usize) -> SMagic {
        SMagic {
            mask,
            magic,
            shift,
            offset,
        }
    }

    pub fn get_index(&self, occupancy: BitBoard) -> usize {
        let blocker = occupancy.0 & self.mask;
        (blocker.wrapping_mul(self.magic) >> self.shift) as usize + self.offset
    }
}

/**
 * Generates random 64-bit integer
 * */
fn random_u64() -> u64 {
    let mut rng = rand::thread_rng();

    let u1: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u2: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u3: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u4: u64 = (rng.gen::<u64>()) & 0xFFFF;

    u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
}

/**
 * Generate random 64-bit integer with a "low" number of bits set
 * */
fn random_u64_fewbits() -> u64 {
    random_u64() & random_u64() & random_u64()
}

fn pop_1st_bit(bb: &mut u64) -> u64 {
    let b = *bb ^ ((bb).wrapping_sub(1));

    let fold: u32 = ((b & 0xffffffff) ^ (b >> 32)) as u32;
    *bb &= bb.wrapping_sub(1);

    BIT_TABLE[((fold.wrapping_mul(0x783A_9B23)) >> 26) as usize]
}

fn index_to_u64(index: u32, bits: u32, m: u64) -> u64 {
    let mut result: u64 = 0;

    let mut new_m: u64 = m;

    for i in 0..bits {
        let j = pop_1st_bit(&mut new_m);
        if (index & (1 << i)) != 0 {
            result |= 1u64 << j;
        }
    }

    result
}

/**
 *
 * Returns a BitBoard where set bits represent the moves a rook can make
 * when on given square. Does not account for blockers
 *
 * Edge squares are excluded because they do not affect attack set
 * */
fn rook_mask(square: u64) -> BitBoard {
    let mut result: u64 = 0;

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    for r in (rank + 1)..7 {
        result |= 1u64 << (file + r * 8);
    }

    for r in (1..=(rank - 1)).rev() {
        result |= 1u64 << (file + r * 8);
    }

    for f in (file + 1)..7 {
        result |= 1u64 << (f + rank * 8);
    }

    for f in (1..=(file - 1)).rev() {
        result |= 1u64 << (f + rank * 8);
    }

    BitBoard(result)
}

/**
 *
 * Returns a BitBoard where set bits represent the moves a bishop can make
 * when on given square. Does not account for blockers
 *
 * Edge squares are excluded because they do not affect attack set
 * */
fn bishop_mask(square: u64) -> BitBoard {
    let mut result: u64 = 0;

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    let mut r = rank + 1;
    let mut f = file + 1;

    while r <= 6 && f <= 6 {
        result |= 1u64 << (f + r * 8);
        r += 1;
        f += 1;
    }

    r = rank + 1;
    f = file - 1;

    while r <= 6 && f >= 1 {
        result |= 1u64 << (f + r * 8);

        r += 1;
        f -= 1;
    }

    r = rank - 1;
    f = file + 1;

    while r >= 1 && f <= 6 {
        result |= 1u64 << (f + r * 8);

        r -= 1;
        f += 1;
    }

    r = rank - 1;
    f = file - 1;

    while r >= 1 && f >= 1 {
        result |= 1u64 << (f + r * 8);

        r -= 1;
        f -= 1;
    }

    BitBoard(result)
}

/**
 *
 * Returns a BitBoard where set bits represent the moves a rook can make
 * when on given square.
 *
 * Accounts for blockers
 * */
fn rook_attack(square: u64, blockers: BitBoard) -> BitBoard {
    let mut result: BitBoard = BitBoard(0);

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    for r in (rank + 1)..8 {
        let mask = BitBoard(1u64 << (file + r * 8));
        result |= mask;
        if blockers & (mask) != EMPTY_BITBOARD {
            break;
        }
    }

    for r in (0..=(rank - 1)).rev() {
        let mask = BitBoard(1u64 << (file + r * 8));
        result |= mask;
        if blockers & (mask) != EMPTY_BITBOARD {
            break;
        }
    }

    for f in (file + 1)..8 {
        let mask = BitBoard(1u64 << (f + rank * 8));
        result |= mask;
        if blockers & mask != EMPTY_BITBOARD {
            break;
        }
    }

    for f in (0..=(file - 1)).rev() {
        let mask = BitBoard(1u64 << (f + rank * 8));
        result |= mask;
        if blockers & mask != EMPTY_BITBOARD {
            break;
        }
    }

    result
}

/**
 *
 * Returns a BitBoard where set bits represent the moves a bishop can make
 * when on given square.
 *
 * Accounts for blockers
 * */
fn bishop_attack(square: u64, blockers: BitBoard) -> BitBoard {
    let mut result: BitBoard = BitBoard(0);

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    let mut r: i32 = rank + 1;
    let mut f: i32 = file + 1;

    while r <= 7 && f <= 7 {
        let mask = BitBoard(1u64 << (f + r * 8));
        result |= mask;

        if blockers & mask != EMPTY_BITBOARD {
            break;
        }
        r += 1;
        f += 1;
    }

    r = rank + 1;
    f = file - 1;

    while r <= 7 && f >= 0 {
        let mask = BitBoard(1u64 << (f + r * 8));
        result |= mask;

        if blockers & mask != EMPTY_BITBOARD {
            break;
        }
        r += 1;
        f -= 1;
    }

    r = rank - 1;
    f = file + 1;

    while r >= 0 && f <= 7 {
        let mask = BitBoard(1u64 << (f + r * 8));
        result |= mask;

        if blockers & mask != EMPTY_BITBOARD {
            break;
        }

        r -= 1;
        f += 1;
    }

    r = rank - 1;
    f = file - 1;

    while r >= 0 && f >= 0 {
        let mask = BitBoard(1u64 << (f + r * 8));
        result |= mask;

        if blockers & mask != EMPTY_BITBOARD {
            break;
        }

        r -= 1;
        f -= 1;
    }

    result
}

/*
 * Returns a Vec of BitBoards that represent the attacks for given slider piece for each
 * subset of the mask BitBoard
 * */
fn generate_attack_map(is_bishop: bool, size: usize, square: u64, mask: BitBoard) -> Vec<BitBoard> {
    let mut map = vec![EMPTY_BITBOARD; size];
    let mut blockers = BitBoard(0);

    // Iterate over all subsets of mask bitboard
    // giving us all the blockers
    // and storing attack bitboards for each blocker BitBoard in the map vec
    for attacks in map.iter_mut() {
        *attacks = if is_bishop {
            bishop_attack(square, blockers)
        } else {
            rook_attack(square, blockers)
        };
        blockers = BitBoard(blockers.0.wrapping_sub(mask.0) & mask.0);
    }
    map
}

struct MagicNumberCollision;

fn collision_detected(actual: &[BitBoard], hash: usize, attacks: BitBoard) -> bool {
    actual[hash] != EMPTY_BITBOARD && actual[hash] != attacks
}

/**
 * Attempts to fill hashmap with magic number
 */
fn try_magic_number(
    mask: BitBoard,
    magic: u64,
    expected: &[BitBoard],
) -> Result<(), MagicNumberCollision> {
    let shift = 64 - mask.0.count_ones();

    let mut actual = vec![EMPTY_BITBOARD; expected.len()];
    let mut occupancies = 0u64;

    for &attacks in expected.iter() {
        let hash = (occupancies.wrapping_mul(magic) >> shift) as usize;

        if collision_detected(&actual, hash, attacks) {
            return Err(MagicNumberCollision);
        }
        actual[hash] = attacks;
        occupancies = occupancies.wrapping_sub(mask.0) & mask.0;
    }

    Ok(())
}

/**
 * Returns MagicEntry for square, depending on bishop or rool
 *
 * panics if magic number not found
 */
fn find_magic(square: u64, is_bishop: bool) -> MagicEntry {
    let mask = if is_bishop {
        bishop_mask(square)
    } else {
        rook_mask(square)
    };

    let ones = mask.0.count_ones();
    let size = 1 << ones;
    let attacks: Vec<BitBoard> = generate_attack_map(is_bishop, size, square, mask);

    // Attempt to find magic number 100_000_000 times
    for _ in 0..100_000_000 {
        // Possible magic number
        let magic = random_u64_fewbits();

        // Skip bad candidates
        if ((mask.0.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones()) < 6 {
            continue;
        }

        // Attempt putting magic number in hashmap
        if try_magic_number(mask, magic, &attacks).is_ok() {
            let shift = 64 - ones;
            let size: usize = attacks.len();

            return MagicEntry::new(mask.0, magic, shift, size);
        }
    }

    panic!("MAGIC NUMBER NOT FOUND");
}

/**
 * Fills attack tables for bishop and rook
 * */
pub fn init_slider_attacks(move_gen: &mut MoveGenerator, is_bishop: bool) {
    for square in 0..64 {
        let bishop_magic: SMagic = BISHOP_MAGICS[square as usize];
        let rook_magic: SMagic = ROOK_MAGICS[square as usize];

        move_gen.bishop_tbl[square as usize] = bishop_magic;
        move_gen.rook_tbl[square as usize] = rook_magic;

        let bit_count: u32 = if is_bishop {
            bishop_magic.mask.count_ones()
        } else {
            rook_magic.mask.count_ones()
        };

        let occupancy_variations = 1 << bit_count;

        for count in 0..occupancy_variations {
            if is_bishop {
                let occupancy = BitBoard(index_to_u64(count, bit_count, bishop_magic.mask));
                let index = bishop_magic.get_index(occupancy);
                move_gen.bishop_moves[index] = bishop_attack(square, occupancy);
            } else {
                let occupancy = BitBoard(index_to_u64(count, bit_count, rook_magic.mask));
                let index = rook_magic.get_index(occupancy);
                move_gen.rook_moves[index] = rook_attack(square, occupancy);
            }
        }
    }
}

pub fn print_magics(is_bishop: bool) {
    let mut offset = 0;
    for square in 0..64 {
        let entry = if is_bishop {
            find_magic(square, true)
        } else {
            find_magic(square, false)
        };

        println!(
            "    MagicEntry {{ mask: 0x{:0>16X}, magic: 0x{:0>16X}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, offset,
        );
        offset += entry.size;
    }

    if is_bishop {
        println!("pub const BISHOP MAP_SIZE: usize = {};", offset);
        println!();
    } else {
        println!("pub const ROOK MAP_SIZE: usize = {};", offset);
        println!();
    }
}
