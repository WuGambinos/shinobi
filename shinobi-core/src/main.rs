use shinobi_core::{Bot, Engine, Position, START_POS, perft::perft, Piece};

fn main() {
    env_logger::init();
    let position = Position::from_fen(START_POS);
    let mut engine = Engine::new(position);

    /*;
    let mut bot: Bot = Bot::new();
    bot.think(&mut engine.position, &engine.move_gen);
    */

    /*
    engine.uci_loop();
    */
}
