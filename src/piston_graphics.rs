use image::{RgbaImage};
use piston::{WindowSettings, Events, EventSettings, RenderEvent, UpdateEvent, EventLoop, ButtonEvent, ButtonState, Button, Key};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, Filter};
use graphics::color::{BLACK, BLUE};
use graphics::{Rectangle, Image, clear, Context};
use std::error::Error;

use thiserror::Error;
use anyhow;
use thiserror::private::AsDynError;

use crate::{RuleSet, ColoredDataType, Game};

#[derive(Error, Debug)]
#[error("Piston  error: {msg}")]
pub struct PistonError {
    msg:String
}


pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), PistonError>
    where R: RuleSet,
          R::Data: ColoredDataType{

    let mut window: GlutinWindow = WindowSettings::new("Game", window_size)
        .exit_on_esc(true)
        .resizable(false)
        .build().map_err(|e|PistonError{msg:e.to_string()})?;

    let mut gl_graph = GlGraphics::new(OpenGL::V3_3);

    let mut events = Events::new(EventSettings::new().ups(30).ups_reset(0).max_fps(30));
    let image = Image::new().rect([0.0,0.0,window_size.0  as f64,window_size.1 as f64]);
    let texture_settings = TextureSettings::new().convert_gamma(false).filter(Filter::Nearest);

    let mut pause = true;
    let mut fps = 0.0;

    let mut texture = Texture::from_image(&game.to_rgba()?, &texture_settings);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){

            let r:Result<(), PistonError> = gl_graph.draw(args.viewport(), |c, gl|{
                clear(BLUE, gl);
                texture.update(&game.to_rgba()?);
                image.draw(&texture, &c.draw_state, c.transform, gl);
                Ok(())
            });
            r?;

        };
        if let Some(args) = e.update_args(){
            fps = 1.0/args.dt;
            if !pause{
                game.next_step();
            }

        }
        if let Some(args) = e.button_args(){
            if args.state == ButtonState::Press{
                match args.button{
                    Button::Keyboard(Key::Space) => pause ^= true,
                    Button::Keyboard(Key::Right) => if pause { game.next_step()},
                    //Button::Keyboard(Key::F) =>
                    _ => ()

                }
            }
        }
    }
    Ok(())

}

trait GenerateRgba{
    fn to_rgba(&self)->Result<RgbaImage, PistonError>;
}

impl<R> GenerateRgba for Game<R>
    where R: RuleSet,
          R::Data: ColoredDataType{
    fn to_rgba(&self) -> Result<RgbaImage, PistonError> {
        RgbaImage::from_raw(self.grid.width as u32, self.grid.height as u32,self.to_raw_colors())
            .ok_or(PistonError {msg:"Cannot create image".to_string()})
    }
}
