mod heat_rules;
mod conway_rules;


use heat_rules::HeatRules;
use conway_rules::{ClassicConway, ConwayWithHistory, ConwayColors};
use game_of_life::Game;


//**************************************************************

fn main() {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 600;
    const SIZE: (u16, u16) = (300,300);

    let mut game: Game<ConwayColors> = Game::init_random(SIZE).unwrap();
    #[cfg(feature = "graphics-ggez")]
    game.run_with_ggez((WIDTH, HEIGHT)).unwrap();

    #[cfg(feature = "graphics-piston")]
    game.run_with_piston((WIDTH, HEIGHT)).unwrap();
}
