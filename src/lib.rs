pub mod bitboard;
pub mod enums;
pub mod constants;
pub mod util;
pub mod position;
pub mod magic;

pub use::rand::prelude::*;
pub use magic::*;
pub use bitboard::*;
pub use enums::*;
pub use macroquad::prelude::*;
pub use macroquad::color::Color;
pub use constants::*;
pub use util::*;
pub use position::*;
pub use strum::IntoEnumIterator;
pub use strum_macros::EnumIter;

