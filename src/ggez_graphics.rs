use ggez::input::keyboard::KeyMods;
use ggez::{ContextBuilder, Context, GameResult, GameError};
use ggez::conf::WindowMode;
use ggez::event::{EventHandler, KeyCode};
use ggez::event::run as ggez_run;
use ggez::{graphics, timer, event};
use ggez::graphics::{Rect, DrawParam, Image, Color};

use crate::{RuleSet, Game, ColoredDataType, GError};
use crate::GError::GgezError;


pub fn run<R:'static>(window_size: (u32, u32), game: Game<R>) -> !
    where R: RuleSet,
          R::Data: ColoredDataType {
    let (mut ctx, event_loop) = ContextBuilder::new("Game of Life", "Eero")
        .window_mode(WindowMode { width: window_size.0 as f32, height: window_size.1 as f32, ..Default::default() })
        .build().unwrap();
    let handler = MyEventHandler::<R>::new(&mut ctx, game).unwrap();
    ggez_run(ctx, event_loop, handler);
}

struct MyEventHandler<R>
    where R: RuleSet {
    game: Game<R>,
    fps: graphics::Text,
    is_pause: bool,
    show_fps: bool,
}

impl<R> MyEventHandler<R>
    where R: RuleSet {
    fn new(ctx: &mut Context, game: Game<R>) -> GameResult<MyEventHandler<R>> {
        graphics::set_screen_coordinates(ctx, Rect::new_i32(0, 0, game.grid.width as i32, game.grid.height as i32))?;
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        Ok(MyEventHandler { game, fps: graphics::Text::new(""), is_pause: true, show_fps: false })
    }
}

impl<R> EventHandler for MyEventHandler<R>
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
        let img = Image::from_rgba8(ctx, self.game.grid.width, self.game.grid.height, &self.game.to_raw_colors())?;

        graphics::clear(ctx, Color::BLACK);
        graphics::draw(ctx, &img, DrawParam::default())?;
        if self.show_fps {
            graphics::draw(ctx, &self.fps, ([0.0, 0.0], Color::WHITE))?;
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