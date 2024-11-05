use shinobi_core::{
    search::Search, search::SearcherEnum, Bot, Engine, Position, SearchInfo, START_POS,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
     //let mut engine = Engine::new(SearcherEnum::Bot(Bot::new()));
    let mut engine = Engine::new(SearcherEnum::Search(Search::new()));
    engine.run();
    Ok(())
}
