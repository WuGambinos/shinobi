use crate::MoveGenerator;
use crate::Position;
pub struct Engine {
    pub position: Position,
    pub move_gen: MoveGenerator,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Position::new(),
            move_gen: MoveGenerator::new(),
        }
    }
}
