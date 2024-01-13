use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use pixels::{Pixels, SurfaceTexture};

use crate::error_handling::GError::PixelsError;
use crate::GError::WinitError;
use crate::{ColoredDataType, GError, Game, RuleSet};

pub fn run<R>(window_size: (u16, u16), game: &mut Game<R>) -> Result<(), GError>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    let event_loop = EventLoop::new().map_err(|e| WinitError { source: e.into() })?;
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(window_size.0 as f64, window_size.1 as f64);
        WindowBuilder::new()
            .with_title("GoL Pixels")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .map_err(|e| WinitError { source: e.into() })?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            game.grid.width as u32,
            game.grid.height as u32,
            surface_texture,
        )
        .map_err(|e| PixelsError { source: e.into() })?
    };
    let mut is_paused = true;
    let mut possible_error = None;

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                pixels.frame_mut().copy_from_slice(&game.to_raw_colors());
                let error = pixels.render().err();
                if error.is_some() {
                    elwt.exit();
                    possible_error = error.map(|e| PixelsError { source: e.into() });
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
        })
        .map_err(|e| WinitError { source: e.into() })?;

    possible_error.map_or(Ok(()), |e| Err(e))
}
