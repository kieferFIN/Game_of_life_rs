extern crate core;

pub use crate::error_handling::GError;
pub use crate::error_handling::GResult;
pub use crate::timer::Timer;

pub use crate::game::Game;

mod error_handling;
mod game;
mod grid;
mod timer;

#[cfg(feature = "graphics-ggez")]
mod ggez_graphics;

#[cfg(feature = "graphics-piston")]
mod piston_graphics;

#[cfg(feature = "graphics-pixels")]
mod pixels_graphics;

#[cfg(feature = "scripting")]
mod scripting;

#[cfg(feature = "graphics-sfml")]
mod sfml_graphics;

#[cfg(feature = "graphics-terminal")]
mod terminal_graphics;

pub mod backends;

type IndexType = (i32, i32);

pub type Color = (u8, u8, u8, u8);

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub trait RuleSet: Clone + Send + Sync + 'static {
    type Data: DataType;
    const SOURCE_SIZE: u8;
    fn next(source: &[&Self::Data]) -> Self::Data;
}

pub trait DataType: Clone + Send + Sync + 'static {}

pub trait ColoredDataType: DataType {
    fn get_color(&self) -> Color;
}

pub trait PrintableDataType: DataType {
    fn get_char(&self) -> char;
}

pub trait RandomInit {
    fn rnd() -> Self;
}
