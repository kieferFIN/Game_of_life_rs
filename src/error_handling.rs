use thiserror::Error;

#[derive(Error, Debug)]
pub enum GError {
    #[error("original data (size: {size}), cannot be {width} wide.")]
    InitializationError { size: usize, width: u16 },
    #[cfg(feature = "graphics-ggez")]
    #[error("Something bad happened in Ggez")]
    GgezError(#[from] ggez::GameError),
    #[cfg(feature = "graphics-piston")]
    #[error("Something bad happened in Piston")]
    PistonError(#[from] crate::piston_graphics::PistonError),
    #[cfg(feature = "graphics-pixels")]
    #[error("Something bad happened in Pixels")]
    PixelsError(#[from] crate::pixels_graphics::PixelsError),
    #[cfg(feature = "graphics-sfml")]
    #[error("Something bad happened in SFML")]
    SfmlError(#[from] crate::sfml_graphics::SfmlError),
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
