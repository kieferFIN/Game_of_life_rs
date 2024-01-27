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

fn main() -> GResult<()> {
    //use game_of_life::RandomInit;

    const WIDTH: u16 = 320;
    const HEIGHT: u16 = 160;
    const SIZE: (u16, u16) = (WIDTH, HEIGHT);

    //let v:Vec<BoolData> = (0..600).map(|_|BoolData::rnd()).collect();

    //let mut game: Game<ClassicConway>  = Game::init_with_data(v,333).context("Data is wrong size")?;

    let mut game: Game<ConwayWithHistory> = Game::init_random_data(SIZE)?;

    game.run::<BackEnd>((WIDTH as u32 * 4, HEIGHT as u32 * 4))
}
