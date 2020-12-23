use game_of_life::{Game, RuleSet, DataType};
use std::collections::VecDeque;
use ggez::{ContextBuilder, Context, GameResult, GameError};
use ggez::conf::WindowMode;
use ggez::event::{EventHandler, run};
use ggez::{graphics, timer};
use ggez::graphics::{Color, DrawMode, Rect, DrawParam, MeshBuilder};
use ggez::nalgebra::Point2;
use std::time::Instant;


trait RandomInit {
    fn rnd() -> Self;
}

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

impl RandomInit for BoolData {
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

impl RandomInit for BoolHist {
    fn rnd() -> BoolHist {
        BoolHist {
            history: VecDeque::from(vec![false; 5]),
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
            let gray = (s * 40) as u8;
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

impl DataType for ColorData{
    fn get_color(&self) -> (u8, u8, u8, u8) {
        (self.r as u8 * 255,self.g as u8 * 255,self.b as u8 * 255, 255)
    }

    fn get_char(&self) -> char {
        if self.r {
            '*'
        }else {
            ' '
        }
    }
}

struct ColorRules{}

impl RuleSet for ColorRules{
    type Data = ColorData;

    fn next(source: &[&ColorData]) -> ColorData {
        let me = source[4];
        let all = source.iter().fold((0,0,0), |acc, d| (acc.0+d.r as i8, acc.1+d.g as i8, acc.2+d.b as i8) );
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

        ColorData{r,g,b}
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
}

impl<R> MyEventHandler<R>
    where R: RuleSet,
          R::Data: RandomInit {
    fn new(ctx: &mut Context, game_size: (u16, u16)) -> GameResult<MyEventHandler<R>> {
        let total_size = game_size.0 as usize * game_size.1 as usize;
        println!("{}", total_size);
        let mut data = Vec::with_capacity(total_size);
        for _ in 0..total_size {
            data.push(R::Data::rnd())
        };
        graphics::set_screen_coordinates(ctx, Rect::new_i32(0, 0, game_size.0 as i32, game_size.1 as i32))?;
        let game = Game::init_with_data(&data, game_size.0).ok_or(GameError::ConfigError("wrong params for game init".into()))?;
        Ok(MyEventHandler { game, game_size, fps: graphics::Text::new("") })
    }
}

impl< R> EventHandler for MyEventHandler< R>
    where R: RuleSet {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.next_step();
        self. fps = graphics::Text::new(format!("{:.2}",timer::fps(ctx)));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut mb = MeshBuilder::new();

        for (c, d) in &self.game {
            let (r, g, b, a) = d.get_color();
            mb.rectangle(
                DrawMode::fill(),
                Rect::new_i32(c.0, c.1, 1, 1),
                Color::from_rgba(r, g, b, a),
            );
        };
        let mesh = mb.build(ctx)?;
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        graphics::draw(ctx, &self.fps, (Point2::new(0.0, 0.0), graphics::WHITE))?;
        graphics::present(ctx)?;
        Ok(())
    }
}


fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const SIZE: (u16, u16) = (800, 600);
/*
    let (mut ctx, mut event_loop) =
        ContextBuilder::new("Game of Life", "Eero")
            .window_mode(WindowMode { width: WIDTH as f32, height: HEIGHT as f32, ..Default::default() })
            .build().unwrap();

    let mut handler = MyEventHandler::<ColorRules>::new(&mut ctx, SIZE).unwrap();
    match run(&mut ctx, &mut event_loop, &mut handler) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e)
    }
    */

    let total_size = SIZE.0 as usize * SIZE.1 as usize;
    let mut vec = Vec::with_capacity(total_size);
    for _ in 0..total_size{
        vec.push(ColorData::rnd());
    };

    let mut game = Game::<ColorRules>::init_with_data(&vec, SIZE.0).unwrap();
    let start = Instant::now();
    for _ in 0..100{
        game.next_step();
    }
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));

}
