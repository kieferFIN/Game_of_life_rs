use ggez::input::keyboard::KeyMods;
use ggez::{ContextBuilder, Context, GameResult, GameError};
use ggez::conf::WindowMode;
use ggez::event::{EventHandler, KeyCode, EventsLoop};
use ggez::event::run as ggez_run;
use ggez::{graphics, timer, event};
use ggez::graphics::{Rect, DrawParam, Image};
use ggez::nalgebra::Point2;

use crate::{RuleSet, Game, RandomInit, ColoredDataType};


pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), String>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Game of Life", "Eero")
        .window_mode(WindowMode { width: window_size.0 as f32, height: window_size.1 as f32, ..Default::default() })
        .build().unwrap();
    let mut handler = MyEventHandler::<R>::new(&mut ctx, game).unwrap();
    ggez_run(&mut ctx, &mut event_loop, &mut handler).map_err(|e| e.to_string())
}

struct MyEventHandler<'a, R>
    where R: RuleSet {
    game: &'a mut Game<R>,
    fps: graphics::Text,
    is_pause: bool,
    show_fps: bool,
}

impl<'a, R> MyEventHandler<'a, R>
    where R: RuleSet {
    fn new(ctx: &mut Context, game: &'a mut Game<R>) -> GameResult<MyEventHandler<'a, R>> {
        graphics::set_screen_coordinates(ctx, Rect::new_i32(0, 0, game.grid.width as i32, game.grid.height as i32))?;
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        Ok(MyEventHandler { game, fps: graphics::Text::new(""), is_pause: true, show_fps: false })
    }
}

impl<'a, R> EventHandler for MyEventHandler<'a, R>
    where R: RuleSet,
          R::Data: ColoredDataType {
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
        let mut v = Vec::with_capacity(self.game.grid.width as usize * self.game.grid.height as usize * 4);
        for (_, d) in self.game.into_iter() {
            let (r, g, b, a) = d.get_color();
            v.push(r);
            v.push(g);
            v.push(b);
            v.push(a);
        }
        let img = Image::from_rgba8(ctx, self.game.grid.width, self.game.grid.height, &v)?;

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
            KeyCode::F => self.show_fps ^= true,
            KeyCode::Right => if self.is_pause {self.game.next_step() },
            _ => ()
        }
    }
}