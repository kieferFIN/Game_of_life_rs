use thiserror::Error;

#[cfg(feature = "ggez")]
use ggez::GameError;

#[cfg(feature = "piston")]
use crate::piston_graphics::PistonError;


#[derive(Error, Debug)]
pub enum GError{
    #[error("original data (size: {size}), cannot be {width} wide.")]
    InitializationError{size: usize, width: u16},
    #[cfg(feature = "ggez")]
    #[error("ggez error")]
    GgezError{source: GameError},
    #[cfg(feature = "piston")]
    #[error("piston error")]
    PistonError{source: PistonError},
}