use game_of_life::{Game, RuleSet, DataType};
use simple::{Window, Event, Key};
use std::time::Duration;
use std::collections::VecDeque;


struct ClassicConeway {}

impl RuleSet<BoolData> for ClassicConeway {
    fn next(&self, source: &[&BoolData]) -> BoolData {
        let me = source[4].value;
        let all: i32 = source.iter().map(|x| x.value as i32).sum();
        let neighbours = all - me as i32;

        match (me, neighbours) {
            (true, 2) | (_, 3) => BoolData { value: true },
            _ => BoolData { value: false }
        }
    }

    fn source_size(&self) -> u8 {
        3
    }
}

#[derive(Clone, Copy)]
struct BoolData {
    value: bool
}

impl BoolData {
    fn rnd() -> BoolData {
        BoolData { value: rand::random::<bool>() }
    }
}

impl DataType for BoolData {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        if self.value {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 255)
        }
    }

    fn get_char(&self) -> char {
        if self.value {
            '*'
        } else {
            '_'
        }
    }
}
//**************************************************************

#[derive(Clone)]
struct BoolHist {
    current: bool,
    history: VecDeque<bool>,
}

impl BoolHist {
    fn rnd() -> BoolHist {
        BoolHist {
            history: VecDeque::from(vec![false; 10]),
            current: rand::random::<bool>(),
        }
    }
}

impl DataType for BoolHist {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        if self.current {
            (255, 255, 255, 255)
        } else {
            let s: i32 = self.history.iter().map(|x| *x as i32).sum();
            let gray = (s * 15) as u8;
            (gray, gray, gray, 255)
        }
    }
    fn get_char(&self) -> char {
        if self.current {
            '*'
        } else {
            '_'
        }
    }
}

struct ClassicHistory {}

impl RuleSet<BoolHist> for ClassicHistory {
    fn next(&self, source: &[&BoolHist]) -> BoolHist {
        let me = source[4];
        let all: i32 = source.iter().map(|x| x.current as i32).sum();
        let neighbours = all - me.current as i32;

        let current = match (me.current, neighbours) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let mut history = me.history.clone();
        history.push_back(me.current);
        history.pop_front();
        BoolHist { current, history }
    }

    fn source_size(&self) -> u8 {
        3
    }
}


fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const NUMBERS: (u32, u32) = (160, 120);
    const TOTAL: usize = (NUMBERS.0 * NUMBERS.1) as usize;
    const SIZE: (u32, u32) = (WIDTH / NUMBERS.0, HEIGHT / NUMBERS.1);
    let mut init_data = Vec::with_capacity(TOTAL);
    for _ in 0..TOTAL {
        init_data.push(BoolHist::rnd());
    }
    let rules = ClassicHistory {};
    let mut game = Game::new(&init_data, NUMBERS.0, rules).unwrap();
    let mut screen = Window::new("Game of life", WIDTH as u16, HEIGHT as u16);


    while screen.next_frame() {
        while screen.has_event() {
            match screen.next_event() {
                Event::Keyboard { is_down: true, key: Key::Escape } => screen.quit(),
                _ => ()
            }
        }
        screen.clear();
        game.draw(&mut screen, SIZE);
        game.next();

        std::thread::sleep(Duration::from_millis(50));
    }
}
