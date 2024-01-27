#[cfg(feature = "ggez")]
use ggez::GameError;
#[cfg(feature = "scripting")]
use rhai::EvalAltResult;
use thiserror::Error;

#[cfg(feature = "piston")]
use crate::piston_graphics::PistonError;

#[derive(Error, Debug)]
pub enum GError {
    #[error("original data (size: {size}), cannot be {width} wide.")]
    InitializationError { size: usize, width: u16 },
    #[cfg(feature = "graphics-ggez")]
    #[error("Something bad happened in Ggez")]
    GgezError(#[from] GameError),
    #[cfg(feature = "graphics-piston")]
    #[error("Something bad happened in Piston")]
    PistonError(#[from] PistonError),
    #[cfg(feature = "graphics-pixels")]
    #[error("Something bad happened in Pixels")]
    PixelsError { source: anyhow::Error },
    #[cfg(feature = "graphics-pixels")]
    #[error("Something bad happened in Winit")]
    WinitError { source: anyhow::Error },
    #[cfg(feature = "scripting")]
    #[error("Something bad happened in script")]
    ScriptError {
        #[from]
        source: Box<EvalAltResult>,
    },
    #[cfg(feature = "graphics-terminal")]
    #[error("Something went wrong in terminal")]
    TerminalError(#[from] std::io::Error),
}

pub type GResult<T> = Result<T, GError>;
