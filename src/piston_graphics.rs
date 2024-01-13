use glutin_window::GlutinWindow;
use graphics::{Image, Text};
use image::RgbaImage;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::{
    Button, ButtonEvent, ButtonState, EventSettings, Events, Key, RenderEvent, UpdateEvent,
    WindowSettings,
};
use std::{collections::VecDeque, time::Instant};

use thiserror::Error;

use crate::{ColoredDataType, Game, RuleSet};

#[derive(Error, Debug)]
#[error("Piston  error: {msg}")]
pub struct PistonError {
    msg: String,
}

pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), PistonError>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    let mut window: GlutinWindow = WindowSettings::new("Game", window_size)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .map_err(|e| PistonError { msg: e.to_string() })?;

    let mut gl_graph = GlGraphics::new(OpenGL::V3_3);

    let mut events = Events::new(EventSettings::new());
    let image = Image::new().rect([0.0, 0.0, window_size.0 as f64, window_size.1 as f64]);
    let texture_settings = TextureSettings::new()
        .convert_gamma(false)
        .filter(Filter::Nearest);
    let text = Text::new_color([1.0, 0.0, 0.0, 1.0], 24);
    let ref mut glyphs = GlyphCache::new("sansation.ttf", (), texture_settings)
        .map_err(|e| PistonError { msg: e.to_string() })?;

    let mut pause = true;
    let mut show_fps = false;
    let mut fps_counter = FpsCounter::new();
    let mut fps = 0;
    let mut texture = Texture::from_image(&game.to_rgba()?, &texture_settings);

    fps_counter.get();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl_graph.draw(args.viewport(), |c, gl| {
                texture.update(&game.to_rgba()?);
                image.draw(&texture, &c.draw_state, c.transform, gl);
                if show_fps {
                    text.draw_pos(
                        &format!("{:.2}", fps),
                        [0.0, 24.0],
                        glyphs,
                        &c.draw_state,
                        c.transform,
                        gl,
                    )
                    .map_err(|e| PistonError { msg: e.to_string() })?;
                }

                Ok(())
            })?;
        };
        if let Some(_) = e.update_args() {
            fps = fps_counter.get();
            if !pause {
                game.next_step();
            }
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                match args.button {
                    Button::Keyboard(Key::Space) => pause ^= true,
                    Button::Keyboard(Key::Right) => {
                        if pause {
                            game.next_step()
                        }
                    }
                    Button::Keyboard(Key::F) => show_fps ^= true,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

trait GenerateRgba {
    fn to_rgba(&self) -> Result<RgbaImage, PistonError>;
}

impl<R> GenerateRgba for Game<R>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    fn to_rgba(&self) -> Result<RgbaImage, PistonError> {
        RgbaImage::from_raw(
            self.grid.width as u32,
            self.grid.height as u32,
            self.to_raw_colors(),
        )
        .ok_or(PistonError {
            msg: "Cannot create image".to_string(),
        })
    }
}

struct FpsCounter {
    samples: VecDeque<Instant>,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            samples: VecDeque::with_capacity(256),
        }
    }

    pub fn get(&mut self) -> usize {
        let now = Instant::now();
        while self
            .samples
            .front()
            .map_or(false, |t| now.duration_since(*t).as_secs_f32() > 1.0)
        {
            self.samples.pop_front();
        }
        self.samples.push_back(now);
        self.samples.len()
    }
}

// impl<E: ToString> From<E> for PistonError {
//     fn from(e: E) -> Self {
//         PistonError { msg: e.to_string() }
//     }
// }
