use crate::mov::Move;
const PV_SIZE: usize = 64;

#[derive(Debug, Clone, Copy)]
pub struct PvInfo {
    pub pv_table: [[Option<Move>; PV_SIZE]; PV_SIZE],
    pub pv_length: [i32; PV_SIZE],
}

impl PvInfo {
    pub fn new() -> PvInfo {
        PvInfo {
            pv_table: [[None; PV_SIZE]; PV_SIZE],
            pv_length: [0; PV_SIZE],
        }
    }
}
