mod heat_rules;
mod conway_rules;
mod rgb;

use anyhow::{Context, Result};

use heat_rules::HeatRules;
use conway_rules::{ClassicConway, BoolData, ConwayWithHistory, ConwayColors};
use rgb::RGBRules;
use game_of_life::Game;


//**************************************************************


fn main() -> Result<()> {
    use game_of_life::RandomInit;

    const WIDTH: u16 = 160;
    const HEIGHT: u16 = 43*2;
    const SIZE: (u16, u16) = (WIDTH, HEIGHT );

    //let v:Vec<BoolData> = (0..600).map(|_|BoolData::rnd()).collect();

    //let mut game: Game<ClassicConway>  = Game::init_with_data(v,333).context("Data is wrong size")?;

    let mut game: Game<ConwayWithHistory> = Game::init_random_data(SIZE).context("Data is wrong size")?;

    game.run((WIDTH, HEIGHT/2)).context("Error when running the game")


}
