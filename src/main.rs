
use game_of_life::{Game, RuleSet};



struct ClassicConeway{}

impl ClassicConeway {}

impl RuleSet<bool> for ClassicConeway {
    fn update(&self, source: &[&bool]) -> bool {
        let me = source[4];
        let all: i32 = source.iter().map(|x| **x as i32).sum();
        let neighbours = all - *me as i32;

        match (me, neighbours) {
            (true, 2) | (_, 3) => true,
            _ => false
        }
    }

    fn source_size(&self) -> u32 {
        3
    }
}

fn main() {
    let rules = ClassicConeway{};
    let mut game = Game::new(&[false; 300 * 300],300,rules).unwrap();
    game.next();
}
