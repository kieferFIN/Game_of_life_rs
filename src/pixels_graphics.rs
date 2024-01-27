use winit::dpi::LogicalSize;
use winit::error::{EventLoopError, OsError};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use pixels::{Pixels, SurfaceTexture};

use crate::{ColoredDataType, Game, RuleSet};
use thiserror::Error;

pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), PixelsError>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    let event_loop = EventLoop::new()?;
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(window_size.0 as f64, window_size.1 as f64);
        WindowBuilder::new()
            .with_title("GoL Pixels")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let size = game.get_size();
        Pixels::new(size.width as u32, size.height as u32, surface_texture)?
    };
    let mut is_paused = true;
    let mut possible_error = None;

    event_loop.run(|event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            pixels.frame_mut().copy_from_slice(&game.to_raw_colors().0);
            let error = pixels.render().err();
            if error.is_some() {
                possible_error = error.map(|e| e.into());
                elwt.exit();
                return;
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        _ => {
            if input.update(&event) {
                if !is_paused {
                    game.next_step();
                }

                if input.key_pressed(KeyCode::Escape)
                    || input.close_requested()
                    || input.destroyed()
                {
                    elwt.exit();
                    return;
                }
                if input.key_pressed(KeyCode::Space) {
                    is_paused ^= true;
                }
                if input.key_pressed(KeyCode::ArrowRight) && is_paused {
                    game.next_step();
                }
                window.request_redraw();
            }
        }
    })?;

    possible_error.map_or(Ok(()), |e| Err(e))
}
#[derive(Error, Debug)]
#[error("Pixels  error: {0}")]
pub enum PixelsError {
    Pixel(#[from] pixels::Error),
    Os(#[from] OsError),
    EventLoop(#[from] EventLoopError),
}
