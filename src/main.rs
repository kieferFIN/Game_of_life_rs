mod heat_rules;
mod conway_rules;


use heat_rules::HeatRules;
use conway_rules::{ClassicConway, ConwayWithHistory, ConwayColors, BoolData};
use game_of_life::Game;


//**************************************************************

use anyhow::{Context, Result};

fn main() -> Result<()> {
    use game_of_life::RandomInit;

    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 300;
    const SIZE: (u16, u16) = (300, 300);

    //let v:Vec<BoolData> = (0..600).map(|_|BoolData::rnd()).collect();

    //let mut game: Game<ClassicConway>  = Game::init_with_data(&v,30).context("Size of data and width of data do not match")?;

    let mut game: Game<ClassicConway> = Game::init_random(SIZE);

    #[cfg(feature = "graphics-ggez")]
        game.run_with_ggez((WIDTH, HEIGHT)).context("Something bad happened in ggez.")?;

    #[cfg(feature = "graphics-piston")]
        game.run_with_piston((WIDTH, HEIGHT)).context("Something bad happened in piston.")?;

    #[cfg(feature = "graphics-pixels")]
        game.run_with_pixels((WIDTH, HEIGHT)).context("Something bad happened in pixels.")?;

    Ok(())
}
