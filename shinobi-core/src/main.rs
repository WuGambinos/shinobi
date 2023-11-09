use log::*;
use shinobi_core::{Engine, Position, START_POS};


fn main() {
    let position = Position::from_fen(START_POS);
    let mut engine = Engine::new(position);

    engine.uci_loop();
}
