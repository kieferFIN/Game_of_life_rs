
use game_of_life::{Game};




fn main() {
    let game = Game::new(&[false; 600 * 600],600).unwrap();
    println!("Hello, world!");
}
