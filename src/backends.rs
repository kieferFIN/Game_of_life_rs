use crate::{ColoredDataType, DataType, GError, Game, RuleSet};

pub trait Backend<R>
where
    R: RuleSet,
    R::Data: DataType,
{
    type ErrorType: Into<GError>;
    fn run(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), Self::ErrorType>;
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
