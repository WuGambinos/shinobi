pub mod bitboard;
pub mod constants;
pub mod enums;
pub mod magic;
pub mod position;
pub mod util;
pub mod generator;

pub use ::rand::prelude::*;
pub use bitboard::*;
pub use constants::*;
pub use enums::*;
pub use magic::*;
pub use position::*;
pub use strum::IntoEnumIterator;
pub use strum_macros::EnumIter;
pub use util::*;
pub use generator::*;

pub use sdl2::event::Event;
pub use sdl2::image::{InitFlag, LoadTexture};
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
pub use sdl2::render::TextureCreator;
pub use sdl2::render::WindowCanvas;
pub use sdl2::video::WindowContext;
pub use sdl2::EventPump;
