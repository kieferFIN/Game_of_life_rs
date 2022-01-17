mod heat_rules;
mod conway_rules;
mod rgb;


use heat_rules::HeatRules;
use conway_rules::{ClassicConway, ConwayWithHistory, ConwayColors, BoolData};
use rgb::RGBRules;
use game_of_life::{Game, GError};


//**************************************************************

use anyhow::{Context, Result};

fn main() -> Result<(),GError> {
    use game_of_life::RandomInit;

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 600;
    const SIZE: (u16, u16) = (300, 300);

    //let v:Vec<BoolData> = (0..600).map(|_|BoolData::rnd()).collect();

    //let mut game: Game<ClassicConway>  = Game::init_with_data(&v,30).context("Size of data and width of data do not match")?;

    let mut game: Game<RGBRules> = Game::init_random_data(SIZE);

    game.run((WIDTH,HEIGHT))
}
