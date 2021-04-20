mod heat_rules;
mod conway_rules;

use game_of_life::{Game, RuleSet, DataType, RandomInit, ColoredDataType, PrintableDataType};
use std::time::Instant;

use std::collections::VecDeque;

use heat_rules::HeatRules;
use conway_rules::{ClassicConway, ConwayWithHistory, ConwayColors};





//**************************************************************

fn main() {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 600;
    const SIZE: (u16, u16) = (300,300);

    let mut game: Game<ConwayColors> = Game::init_random(SIZE).unwrap();
    #[cfg(feature = "ggez")]
    game.run_with_ggez((WIDTH, HEIGHT)).unwrap();
}
