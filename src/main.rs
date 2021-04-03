use game_of_life::{Game, RuleSet, DataType, RandomInit, ColoredDataType, PrintableDataType};
use ggez::{ContextBuilder, Context, GameResult, GameError};
use ggez::conf::WindowMode;
use ggez::event::{EventHandler, KeyCode, run};
use ggez::{graphics, timer, event};
use ggez::graphics::{Rect, DrawParam, Image};
use ggez::nalgebra::Point2;
use std::time::Instant;
use ggez::input::keyboard::KeyMods;
use std::collections::VecDeque;


struct ClassicConeway {}

impl RuleSet for ClassicConeway {
    type Data = BoolData;

    fn next(source: &[&BoolData]) -> BoolData {
        let me = source[4].value;
        let all: i32 = source.iter().map(|x| x.value as i32).sum();
        let neighbours = all - me as i32;

        match (me, neighbours) {
            (true, 2) | (_, 3) => BoolData { value: true },
            _ => BoolData { value: false }
        }
    }

    fn source_size() -> u8 {
        3
    }
}

#[derive(Clone)]
struct BoolData {
    value: bool
}

impl DataType for BoolData{}

impl RandomInit for BoolData {
    fn rnd() -> BoolData {
        BoolData { value: rand::random::<bool>() }
    }
}

impl ColoredDataType for BoolData {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        if self.value {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 255)
        }
    }
}

impl PrintableDataType for BoolData{
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

impl RandomInit for BoolHist {
    fn rnd() -> BoolHist {
        BoolHist {
            history: VecDeque::from(vec![false; 5]),
            current: rand::random::<bool>(),
        }
    }
}

impl DataType for BoolHist {}

impl ColoredDataType for BoolHist {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        if self.current {
            (255, 255, 255, 255)
        } else {
            let s: i32 = self.history.iter().map(|x| *x as i32).sum();
            let gray = (s * 40) as u8;
            (gray, gray, gray, 255)
        }
    }
}

struct ClassicHistory {}

impl RuleSet for ClassicHistory {
    type Data = BoolHist;

    fn next(source: &[&BoolHist]) -> BoolHist {
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

    fn source_size() -> u8 {
        3
    }
}
//**************************************************************
#[derive(Clone)]
struct ColorData {
    r: bool,
    g: bool,
    b: bool,
}

impl RandomInit for ColorData {
    fn rnd() -> Self {
        ColorData { r: rand::random::<bool>(), g: rand::random::<bool>(), b: rand::random::<bool>() }
    }
}

impl DataType for ColorData {}

impl ColoredDataType for ColorData {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        (self.r as u8 * 255, self.g as u8 * 255, self.b as u8 * 255, 255)
    }
}

struct ColorRules {}

impl RuleSet for ColorRules {
    type Data = ColorData;

    fn next(source: &[&ColorData]) -> ColorData {
        let me = source[4];
        let all = source.iter().fold((0, 0, 0), |acc, d| (acc.0 + d.r as i8, acc.1 + d.g as i8, acc.2 + d.b as i8));
        let neighbours = (all.0 - me.r as i8, all.1 - me.g as i8, all.2 - me.b as i8);

        let r = match (me.r, neighbours.0) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let g = match (me.g, neighbours.1) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let b = match (me.b, neighbours.2) {
            (true, 2) | (_, 3) => true,
            _ => false
        };

        ColorData { r, g, b }
    }

    fn source_size() -> u8 {
        3
    }
}

//**************************************************************

struct MyEventHandler<R>
    where R: RuleSet {
    game: Game<R>,
    game_size: (u16, u16),
    fps: graphics::Text,
    is_pause: bool,
    show_fps: bool,
}

impl<R> MyEventHandler<R>
    where R: RuleSet,
          R::Data: RandomInit {
    fn new(ctx: &mut Context, game_size: (u16, u16)) -> GameResult<MyEventHandler<R>> {
        graphics::set_screen_coordinates(ctx, Rect::new_i32(0, 0, game_size.0 as i32, game_size.1 as i32))?;
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        let game = Game::init_random(game_size).ok_or(GameError::ConfigError("wrong params for game init".into()))?;
        Ok(MyEventHandler { game, game_size, fps: graphics::Text::new(""), is_pause: false, show_fps: false })
    }
}

impl<R> EventHandler for MyEventHandler<R>
    where R: RuleSet,
          R::Data: RandomInit + ColoredDataType {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.is_pause {
            self.game.next_step();
        }
        if self.show_fps {
            self.fps = graphics::Text::new(format!("{:.2}", timer::fps(ctx)));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut v = Vec::with_capacity(self.game_size.0 as usize * self.game_size.1 as usize * 4);
        for (_, d) in &self.game {
            let (r, g, b, a) = d.get_color();
            v.push(r);
            v.push(g);
            v.push(b);
            v.push(a);
        }
        let img = Image::from_rgba8(ctx, self.game_size.0, self.game_size.1, &v)?;

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &img, DrawParam::default())?;
        if self.show_fps {
            graphics::draw(ctx, &self.fps, (Point2::new(0.0, 0.0), graphics::WHITE))?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Space => self.is_pause ^= true,
            KeyCode::Escape => event::quit(ctx),
            KeyCode::Return => self.game = Game::init_random(self.game_size).unwrap(),
            KeyCode::F => self.show_fps ^= true,
            _ => ()
        }
    }
}


fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const SIZE: (u16, u16) = (400, 300);

    let (mut ctx, mut event_loop) =
        ContextBuilder::new("Game of Life", "Eero")
            .window_mode(WindowMode { width: WIDTH as f32, height: HEIGHT as f32, ..Default::default() })
            .build().unwrap();

    let mut handler = MyEventHandler::<ColorRules>::new(&mut ctx, SIZE).unwrap();
    match run(&mut ctx, &mut event_loop, &mut handler) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e)
    }

}
