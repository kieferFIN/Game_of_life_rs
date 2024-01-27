mod conway_rules;
mod heat_rules;
mod rgb;

use conway_rules::{BoolData, ClassicConway, ConwayColors, ConwayWithHistory};
use game_of_life::{GResult, Game};
use heat_rules::HeatRules;
use rgb::RGBRules;

//**************************************************************

#[cfg(feature = "graphics-terminal")]
type BackEnd = game_of_life::backends::TerminalBackend;

#[cfg(feature = "graphics-sfml")]
type BackEnd = game_of_life::backends::SfmlBackend;

#[cfg(feature = "graphics-pixels")]
type BackEnd = game_of_life::backends::PixelsBackend;

#[cfg(feature = "graphics-ggez")]
type BackEnd = game_of_life::backends::GgezBackend;

#[cfg(feature = "graphics-piston")]
type BackEnd = game_of_life::backends::PistonBackend;

fn main() -> GResult<()> {
    //use game_of_life::RandomInit;

    const WIDTH: u16 = 320;
    const HEIGHT: u16 = 160;
    const SIZE: (u16, u16) = (WIDTH, HEIGHT);

    //let v:Vec<BoolData> = (0..600).map(|_|BoolData::rnd()).collect();

    //let mut game: Game<ClassicConway>  = Game::init_with_data(v,333).context("Data is wrong size")?;

    let mut game: Game<ConwayWithHistory> = Game::init_random_data(SIZE)?;

    #[cfg(not(feature = "graphics-ggez"))]
    let return_value = game.run::<BackEnd>((WIDTH as u32 * 4, HEIGHT as u32 * 4));

    #[cfg(feature = "graphics-ggez")]
    let return_value = game.run_owned::<BackEnd>((WIDTH as u32 * 4, HEIGHT as u32 * 4));

    return_value
}
