use crate::Move;

const HASH_SIZE: usize = 0x400000;
const HASH_FLAG_EXACT: i32 = 0;
const HASH_FLAG_ALPHA: i32 = 1;
const HASH_FLAG_BETA: i32 = 2;

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
