use rand::prelude::*;

pub const BISHOP_BITS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

pub const ROOK_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

pub const BIT_TABLE: [u64; 64] = [
    63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2, 51, 21, 43, 45, 10,
    18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52, 26, 60, 6, 23, 44, 46, 27, 56, 16, 7,
    39, 48, 24, 59, 14, 12, 55, 38, 28, 58, 20, 37, 17, 36, 8,
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

pub fn count_1s(b: u64) -> u64 {
    let mut r = 0;
    let mut num = b;

    while num > 0 {
        num &= num - 1;
        r += 1;
    }

    return r;
}

pub fn pop_1st_bit(bb: &mut u64) -> u64 {
    let b = *bb ^ ((bb).wrapping_sub(1));

    let fold: u32 = ((b & 0xffffffff) ^ (b >> 32)) as u32;
    *bb &= bb.wrapping_sub(1);

    return BIT_TABLE[((fold.wrapping_mul(0x783A_9B23)) >> 26) as usize];
}

pub fn index_to_u64(index: u32, bits: u32, m: &mut u64) -> u64 {
    let mut result: u64 = 0;

    for i in 0..bits {
        let j = pop_1st_bit(m);
        if (index & (1 << i)) == 1 {
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

    let rank = square / 8;
    let file = square % 8;

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
        if block & (mask) == 1 {
            break;
        }
    }

    for r in (1..=(rank - 1)).rev() {
        let mask = 1u64 << (file + r * 8);
        result |= mask;
        if block & (mask) == 1 {
            break;
        }
    }

    for f in (file + 1)..8 {
        let mask = 1u64 << (f + rank * 8);
        result |= mask;
        if block & mask == 1 {
            break;
        }
    }
    for f in (1..=(file - 1)).rev() {
        let mask = 1u64 << (f + rank * 8);
        result |= mask;
        if block & mask == 1 {
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

        if block & mask == 1 {
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

        if block & mask == 1 {
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

        if block & mask == 1 {
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

        if block & mask == 1 {
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
    let mut mask = if bishop == 1 {
        bishop_mask(square)
    } else {
        rook_mask(square)
    };

    let mut a: [u64; 4096] = [0; 4096];
    let mut b: [u64; 4096] = [0; 4096];
    let mut used: [u64; 4096] = [0; 4096];

    let n = count_1s(mask);

    let mut i = 0;
    while i < (1 << n) {
        b[i] = index_to_u64(i as u32, n as u32, &mut mask);
        a[i] = if bishop == 1 {
            bishop_attack(square, b[i])
        } else {
            rook_attack(square, b[i])
        };

        i += 1;
    }

    for k in 0..100000000 {
        let magic = random_u64_fewbits();

        if (count_1s(mask * magic) & 0xFF00000000000000) < 6 {
            continue;
        }

        for i in 0..4096 {
            used[i] = 0;
        }

        i = 0;
        let mut fail = 0;
        while fail == 0 && (i < (1 << n)) {
            let j: u32 = transform(b[i], magic, m);

            if used[j as usize] == 0 {
                used[j as usize] = a[i];
            } else if used[j as usize] != a[i] {
                fail = 1;
            }

            if fail == 0 {
                return magic;
            }
            i += 1;
        }
    }

    println!("*** FAILED ***\n");
    return 0;
}
