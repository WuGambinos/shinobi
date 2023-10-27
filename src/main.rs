use inquire::error::InquireResult;
use log::*;
use shinobi::app;

fn main() -> InquireResult<()> {
    env_logger::init();
    info!("SHINOBI");
    app::start()
}
