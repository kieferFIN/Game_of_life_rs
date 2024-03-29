use crate::{ColoredDataType, DataType, GError, Game, RuleSet};

pub trait Backend<R>
where
    R: RuleSet,
    R::Data: DataType,
{
    type ErrorType: Into<GError>;
    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType>;
}

pub trait BackendStatic<R>
where
    R: RuleSet,
    R::Data: DataType,
{
    type ErrorType: Into<GError>;
    fn run(window_size: (u32, u32), game: Game<R>) -> Result<(), Self::ErrorType>;
}

#[cfg(feature = "graphics-terminal")]
pub struct TerminalBackend {}

#[cfg(feature = "graphics-terminal")]
impl<R> Backend<R> for TerminalBackend
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    type ErrorType = std::io::Error;

    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType> {
        let size = (
            window_size.0.try_into().unwrap(),
            window_size.0.try_into().unwrap(),
        );
        crate::terminal_graphics::run(size, game)
    }
}

#[cfg(feature = "graphics-sfml")]
pub struct SfmlBackend {}

#[cfg(feature = "graphics-sfml")]
impl<R> Backend<R> for SfmlBackend
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    type ErrorType = crate::sfml_graphics::SfmlError;

    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType> {
        crate::sfml_graphics::run(window_size, game)
    }
}

#[cfg(feature = "graphics-pixels")]
pub struct PixelsBackend {}

#[cfg(feature = "graphics-pixels")]
impl<R> Backend<R> for PixelsBackend
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    type ErrorType = crate::pixels_graphics::PixelsError;

    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType> {
        crate::pixels_graphics::run(window_size, game)
    }
}

#[cfg(feature = "graphics-ggez")]
pub struct GgezBackend {}

#[cfg(feature = "graphics-ggez")]
impl<R> BackendStatic<R> for GgezBackend
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    type ErrorType = ggez::error::GameError;

    fn run(window_size: (u32, u32), game: Game<R>) -> Result<(), Self::ErrorType> {
        crate::ggez_graphics::run(window_size, game)
    }
}

#[cfg(feature = "graphics-piston")]
pub struct PistonBackend {}

#[cfg(feature = "graphics-piston")]
impl<R> Backend<R> for PistonBackend
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    type ErrorType = crate::piston_graphics::PistonError;

    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType> {
        crate::piston_graphics::run(window_size, game)
    }
}
