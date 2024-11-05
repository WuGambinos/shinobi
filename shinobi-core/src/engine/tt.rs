
//pub const HASH_SIZE: u64 = 0x400000;
pub const HASH_SIZE: u64 = 0x200;
pub const HASH_FLAG_EXACT: i32 = 0;
pub const HASH_FLAG_ALPHA: i32 = 1;
pub const HASH_FLAG_BETA: i32 = 2;

#[derive(Clone, Copy)]
struct Entry {
    key: u64,
    depth: i32,
    flags: i32,
    value: i32,
    // best: Move,
}

pub struct TT {
    table: [Option<Entry>; HASH_SIZE as usize],
}

impl TT {
    pub fn new() -> TT {
        TT {
            table: [None; HASH_SIZE as usize],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..HASH_SIZE {
            self.table[i as usize] = None;
        }
    }

    pub fn read_hash_entry(&self, hash_key: u64, depth: i32, alpha: i32, beta: i32) -> i32 {
        let hash_entry = self.table[(hash_key % HASH_SIZE) as usize];

        if let Some(entry) = hash_entry {
            if entry.key == hash_key {
                if entry.depth >= depth {
                    if entry.flags == HASH_FLAG_EXACT {
                        println!("EXACT SCORE: ");
                        return entry.value;
                    }

                    if (entry.flags == HASH_FLAG_ALPHA) && (entry.value <= alpha) {
                        println!("ALPHA SCORE: ");
                        return alpha;
                    }

                    if (entry.flags == HASH_FLAG_BETA) && (entry.value >= beta) {
                        println!("BETA SCORE: {}", beta);
                        return beta;
                    }
                }
            }
        }

        println!("NOT FOUND");
        return 100000;
    }

    pub fn write_hash_entry(&mut self, hash_key: u64, depth: i32, val: i32, hash_flag: i32) {
        let hash_entry = Entry {
            key: hash_key,
            depth,
            flags: hash_flag,
            value: val,
        };

        self.table[(hash_key % HASH_SIZE) as usize] = Some(hash_entry);
    }
}

/*
#[derive(Clone, Copy)]
pub struct Entry {
    hash: u64,
    depth: i32,
    flags: i32,
    score: i32,
    best: Move,
}
pub struct TT {
    table: [Option<Entry>; HASH_SIZE],
}

impl TT {

    pub fn probe_hash(&self, depth: i32, alpha: i32, beta: i32) {


    }
    pub fn put(&mut self, hash: u64, entry: Entry) {}

    pub fn get(&self, hash: u64) -> Option<Entry> {
        let index = hash as usize % self.table.len();
        let entry = self.table[index];

        if entry.is_some() && entry.unwrap().hash == hash {
            return entry;
        }

        None
    }

    pub fn clear(&mut self) {
        for i in 0..self.table.len() {
            self.table[i] = None;
        }
    }
}
*/
