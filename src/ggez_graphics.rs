use ggez::conf::WindowMode;
use ggez::event::run as ggez_run;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Image, ImageFormat, Rect, Sampler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};

use crate::{ColoredDataType, Game, RuleSet};

pub fn run<R: 'static>(window_size: (u32, u32), game: Game<R>) -> GameResult<()>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    let (ctx, event_loop) = ContextBuilder::new("Game of Life", "Eero")
        .window_mode(WindowMode {
            width: window_size.0 as f32,
            height: window_size.1 as f32,
            ..Default::default()
        })
        .build()
        .unwrap();
    let handler = MyEventHandler::<R>::new(game);
    ggez_run(ctx, event_loop, handler);
}

struct MyEventHandler<R>
where
    R: RuleSet,
{
    game: Game<R>,
    fps: graphics::Text,
    is_pause: bool,
    show_fps: bool,
    screen_coords: Rect,
}

impl<R> MyEventHandler<R>
where
    R: RuleSet,
{
    fn new(game: Game<R>) -> MyEventHandler<R> {
        let size = game.get_size();
        let coords = Rect::new_i32(0, 0, size.width as i32, size.height as i32);
        MyEventHandler {
            game,
            fps: graphics::Text::new(""),
            is_pause: true,
            show_fps: false,
            screen_coords: coords,
        }
    }
}

impl<R> EventHandler for MyEventHandler<R>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.is_pause {
            self.game.next_step();
        }
        if self.show_fps {
            self.fps = graphics::Text::new(format!("{:.2}", ctx.time.fps()));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (data, size) = self.game.to_raw_colors();
        let img = Image::from_pixels(
            ctx,
            &data,
            ImageFormat::Rgba8Unorm,
            size.width as u32,
            size.height as u32,
        );
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_screen_coordinates(self.screen_coords);
        canvas.set_sampler(Sampler::nearest_clamp());

        canvas.draw(&img, DrawParam::new());
        if self.show_fps {
            canvas.draw(
                &self.fps,
                DrawParam::new().color(Color::WHITE).dest([0.0, 0.0]),
            );
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult<()> {
        match input.keycode {
            Some(KeyCode::Space) => self.is_pause ^= true,
            Some(KeyCode::Escape) => ctx.request_quit(),
            Some(KeyCode::F) => self.show_fps ^= true,
            Some(KeyCode::Right) => {
                if self.is_pause {
                    self.game.next_step()
                }
            }
            _ => (),
        };
        Ok(())
    }
}
