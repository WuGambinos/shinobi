pub mod app;
pub mod util;
use inquire::error::InquireResult;
use log::*;

fn main() -> InquireResult<()> {
    env_logger::init();
    info!("SHINOBI");
    app::start()
}
