use crate::BitBoard;
use crate::Position;
use crate::SMagic;
use rand::prelude::*;

#[rustfmt::skip]
pub const BISHOP_BITS: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6,
];

#[rustfmt::skip]
pub const ROOK_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[rustfmt::skip]
pub const BIT_TABLE: [u64; 64] = [
    63, 30, 3 , 32, 25, 41, 22, 33,
    15, 50, 42, 13, 11, 53, 19, 34,
    61, 29, 2 , 51, 21, 43, 45, 10,
    18, 47, 1 , 54, 9 , 57, 0 , 35,
    62, 31, 40, 4 , 49, 5 , 52, 26,
    60, 6 , 23, 44, 46, 27, 56, 16,
    7 , 39, 48, 24, 59, 14, 12, 55,
    38, 28, 58, 20, 37, 17, 36, 8 ,
];

pub const BISHOP_MAGICS: [u64; 64] = [
    0x4041620809010010,
    0x10500101202102,
    0x24202004108001A,
    0x108260040011000,
    0x401104000000000,
    0x822011008062000,
    0x10311808020A0002,
    0x30420201014100,
    0x852041082280110,
    0xA00214011020,
    0x84012241400A450,
    0x1300822082000805,
    0x1241C10284008C0,
    0x100A2804C02224,
    0x40202411C40A0,
    0x6620202010410,
    0x201080A020429090,
    0x15010208280300,
    0x422040102020202,
    0x208020082830080,
    0x804200202010014,
    0x1110200110101000,
    0x11000401015000,
    0x2001000024010410,
    0x84000880208C8,
    0x2402280802080820,
    0x2080C0006002602,
    0x2008008008102,
    0x803010000444000,
    0x201020008405004,
    0x2844204084821088,
    0xC0484021010800,
    0x20C10480014A024,
    0x24040400031040,
    0x2009040210010804,
    0x8000200800090106,
    0x1811010401420020,
    0x1E05080121020200,
    0x28820880040894,
    0x10C108200023500,
    0x50900808C22011,
    0x6520881110110901,
    0x7010801080200,
    0x20B0012018043100,
    0x1446082100404405,
    0x4C81020081001200,
    0x1888C80800800044,
    0x510010101080020,
    0x500400A290100000,
    0xC001006110080008,
    0x20100886140,
    0x80002042020000,
    0xA000101002088044,
    0x8311100210010302,
    0xC804084204040040,
    0x9010102160188,
    0x400621004A304004,
    0x8000044C02011000,
    0x1004090080480800,
    0x1808008460810,
    0xA489500090320880,
    0x8002000450820200,
    0x200850404280200,
    0x602040414084202,
];

pub const ROOK_MAGICS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];

pub fn random_u64() -> u64 {
    let mut rng = rand::thread_rng();

    let u1: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u2: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u3: u64 = (rng.gen::<u64>()) & 0xFFFF;
    let u4: u64 = (rng.gen::<u64>()) & 0xFFFF;

    return u1 | (u2 << 16) | (u3 << 32) | (u4 << 48);
}

pub fn random_u64_fewbits() -> u64 {
    return random_u64() & random_u64() & random_u64();
}

pub fn pop_1st_bit(bb: &mut u64) -> u64 {
    let b = *bb ^ ((bb).wrapping_sub(1));

    let fold: u32 = ((b & 0xffffffff) ^ (b >> 32)) as u32;
    *bb &= bb.wrapping_sub(1);

    return BIT_TABLE[((fold.wrapping_mul(0x783A_9B23)) >> 26) as usize];
}

pub fn index_to_u64(index: u32, bits: u32, m: u64) -> u64 {
    let mut result: u64 = 0;

    let mut new_m: u64 = m;

    for i in 0..bits {
        let j = pop_1st_bit(&mut new_m);
        if (index & (1 << i)) != 0 {
            result |= 1u64 << j;
        }
    }

    return result;
}

pub fn rook_mask(square: u64) -> u64 {
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

    return result;
}

pub fn bishop_mask(square: u64) -> u64 {
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

    return result;
}

pub fn rook_attack(square: u64, block: u64) -> u64 {
    let mut result: u64 = 0;

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    for r in (rank + 1)..8 {
        let mask = 1u64 << (file + r * 8);
        result |= mask;
        if block & (mask) != 0 {
            break;
        }
    }

    for r in (0..=(rank - 1)).rev() {
        let mask = 1u64 << (file + r * 8);
        result |= mask;
        if block & (mask) != 0 {
            break;
        }
    }

    for f in (file + 1)..8 {
        let mask = 1u64 << (f + rank * 8);
        result |= mask;
        if block & mask != 0 {
            break;
        }
    }

    for f in (0..=(file - 1)).rev() {
        let mask = 1u64 << (f + rank * 8);
        result |= mask;
        if block & mask != 0 {
            break;
        }
    }

    return result;
}

pub fn bishop_attack(square: u64, block: u64) -> u64 {
    let mut result: u64 = 0;

    let rank: i32 = (square / 8) as i32;
    let file: i32 = (square % 8) as i32;

    let mut r: i32 = rank + 1;
    let mut f: i32 = file + 1;

    while r <= 7 && f <= 7 {
        let mask = 1u64 << (f + r * 8);
        result |= mask;

        if block & mask != 0 {
            break;
        }
        r += 1;
        f += 1;
    }

    r = rank + 1;
    f = file - 1;

    while r <= 7 && f >= 0 {
        let mask = 1u64 << (f + r * 8);
        result |= mask;

        if block & mask != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }

    r = rank - 1;
    f = file + 1;

    while r >= 0 && f <= 7 {
        let mask = 1u64 << (f + r * 8);
        result |= mask;

        if block & mask != 0 {
            break;
        }

        r -= 1;
        f += 1;
    }

    r = rank - 1;
    f = file - 1;

    while r >= 0 && f >= 0 {
        let mask = 1u64 << (f + r * 8);
        result |= mask;

        if block & mask != 0 {
            break;
        }

        r -= 1;
        f -= 1;
    }

    return result;
}

pub fn transform(b: u64, magic: u64, bits: u32) -> u32 {
    return ((b * magic) >> (64 - bits)) as u32;
}

pub fn find_magic(square: u64, m: u32, bishop: u64) -> u64 {
    let mask = if bishop == 1 {
        bishop_mask(square)
    } else {
        rook_mask(square)
    };

    let mut attacks: [u64; 4096] = [0; 4096];
    let mut occupancies: [u64; 4096] = [0; 4096];
    let mut used_attacks: [u64; 4096] = [0; 4096];

    let ones = mask.count_ones();
    let num_occupancy_permutations = 1 << ones;

    for i in 0..num_occupancy_permutations {
        // Fill occupancies
        occupancies[i] = index_to_u64(i as u32, ones, mask);

        // Fill attacks
        attacks[i] = if bishop == 1 {
            bishop_attack(square, occupancies[i])
        } else {
            rook_attack(square, occupancies[i])
        };
    }

    for _ in 0..100000000 {
        // Possible magic number
        let magic = random_u64_fewbits();

        // Skip bad candidates
        if ((mask.wrapping_mul(magic) & 0xFF00000000000000).count_ones()) < 6 {
            continue;
        }

        used_attacks.fill(0);

        let mut i = 0;
        let mut fail = 0;

        while fail == 0 && i < num_occupancy_permutations {
            // Get magic index
            let magic_index: u32 = transform(occupancies[i], magic, m);

            // If open index
            if used_attacks[magic_index as usize] == 0 {
                used_attacks[magic_index as usize] = attacks[i];
            }
            // Collision
            else if used_attacks[magic_index as usize] != attacks[i] {
                fail = 1;
            }

            // Valid magic number
            if fail == 0 {
                return magic;
            }
            i += 1;
        }
    }

    println!("*** FAILED ***\n");
    return 0;
}

pub fn print_magic_numbers(bishop: bool) {
    for square in 0..64 {
        if bishop {
            println!("{:#X}", find_magic(square, BISHOP_BITS[square as usize], 1));
        } else {
            println!("{:#X}", find_magic(square, ROOK_BITS[square as usize], 0));
        }
    }
}

pub fn init_slider_attacks(position: &mut Position, is_bishop: bool) {
    // Iterate over board
    for square in 0..64 {
        let bishop_smagic: SMagic =
            SMagic::new(bishop_mask(square), BISHOP_MAGICS[square as usize]);
        let rook_smagic: SMagic = SMagic::new(rook_mask(square), ROOK_MAGICS[square as usize]);

        // Fill rook and bishop Table
        position.bishop_tbl[square as usize] = bishop_smagic;
        position.rook_tbl[square as usize] = rook_smagic;

        // Initalize current mask
        let mask: u64 = if is_bishop {
            bishop_mask(square)
        } else {
            rook_mask(square)
        };

        // Count attack mask bits
        let bit_count: u32 = mask.count_ones();

        // Occupancy variations count
        let occupancy_variations = 1 << bit_count;

        for count in 0..occupancy_variations {
            if is_bishop {
                let occupancy = index_to_u64(count, bit_count, mask);
                let magic_index = occupancy.wrapping_mul(BISHOP_MAGICS[square as usize])
                    >> 64 - BISHOP_BITS[square as usize];

                position.bishop_attacks[square as usize][magic_index as usize] =
                    BitBoard(bishop_attack(square, occupancy));
            } else {
                let occupancy = index_to_u64(count, bit_count, mask);
                let magic_index = occupancy.wrapping_mul(ROOK_MAGICS[square as usize])
                    >> 64 - ROOK_BITS[square as usize];

                position.rook_attacks[square as usize][magic_index as usize] =
                    BitBoard(rook_attack(square, occupancy));
            }
        }
    }
}

pub fn get_bishop_attacks(position: &Position, square: u64, occupancy: u64) -> u64 {
    let mut occ = occupancy;

    occ &= position.bishop_tbl[square as usize].mask.0;
    occ = occ.wrapping_mul(position.bishop_tbl[square as usize].magic.0);
    occ >>= 64 - BISHOP_BITS[square as usize];

    return position.bishop_attacks[square as usize][occ as usize].0;
}

pub fn get_rook_attacks(position: &Position, square: u64, occupancy: u64) -> u64 {
    let mut occ = occupancy;
    occ &= position.rook_tbl[square as usize].mask.0;
    occ = occ.wrapping_mul(position.rook_tbl[square as usize].magic.0);
    occ >>= 64 - ROOK_BITS[square as usize];

    return position.rook_attacks[square as usize][occ as usize].0;
}

pub fn get_queen_attacks(position: &Position, square: u64, occupancy: u64) -> u64 {
    return get_rook_attacks(position, square, occupancy)
        | get_bishop_attacks(position, square, occupancy);
}
