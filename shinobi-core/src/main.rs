use shinobi_core::{perft::perft, print_magics, Bot, Engine, Piece, Position, START_POS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let position = Position::from_fen(START_POS)?;
    let mut engine = Engine::new(position);

    /*
    let mut bot: Bot = Bot::new();
    bot.think(&mut engine.position, &engine.move_gen);
    */

    engine.uci_loop();

    Ok(())
}
