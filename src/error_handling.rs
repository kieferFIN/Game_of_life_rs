use thiserror::Error;

#[cfg(feature = "ggez")]
use ggez::GameError;

#[cfg(feature = "piston")]
use crate::piston_graphics::PistonError;

#[cfg(feature = "scripting")]
use rhai::EvalAltResult;


#[derive(Error, Debug)]
pub enum GError {
    #[error("original data (size: {size}), cannot be {width} wide.")]
    InitializationError { size: usize, width: u16 },
    #[cfg(feature = "graphics-ggez")]
    #[error("Something bad happened in Ggez")]
    GgezError { source: GameError },
    #[cfg(feature = "graphics-piston")]
    #[error("Something bad happened in Piton")]
    PistonError { source: PistonError },
    #[cfg(feature = "graphics-pixels")]
    #[error("Something bad happened in Pixels")]
    PixelsError { source: anyhow::Error},
    #[cfg(feature = "graphics-pixels")]
    #[error("Something bad happened in Winit")]
    WinitError { source: anyhow::Error },
    #[cfg(feature = "scripting")]
    #[error("Something bad happened in script")]
    ScriptError{source: Box<EvalAltResult>},
}

#[cfg(feature = "scripting")]
impl From<Box<EvalAltResult>> for GError{
    fn from(e: Box<EvalAltResult>) -> Self {
        GError::ScriptError {source:e}
    }
}

#[cfg(feature = "piston")]
impl From<PistonError> for GError{
    fn from(e: PistonError) -> Self {
        GError::PistonError {source:e}
    }
}