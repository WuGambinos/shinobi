use shinobi_core::{Engine, Position, START_POS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut engine = Engine::new();
    engine.uci_loop();
    Ok(())
}
