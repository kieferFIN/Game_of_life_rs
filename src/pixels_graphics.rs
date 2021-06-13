use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;
use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use winit::platform::run_return::EventLoopExtRunReturn;

use pixels::{SurfaceTexture, Pixels};

use crate::{RuleSet, ColoredDataType, Game, GError};
use crate::GError::WinitError;
use crate::error_handling::GError::PixelsError;



pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType{
    let mut event_loop =EventLoop::new();
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
        Pixels::new(game.grid.width as u32, game.grid.height as u32, surface_texture).map_err(|e| PixelsError {source:e.into()})?
    };
    let mut possible_error = None;

    event_loop.run_return( |event,_,control_flow|{
        if let Event::RedrawRequested(_) = event{
            pixels.get_frame().copy_from_slice(&game.to_raw_colors());
            let error = pixels.render().err();
            if error.is_some(){
                *control_flow = ControlFlow::Exit;
                possible_error = error.map(|e|PixelsError {source:e.into()});
                return;
            }
            if input.update(&event){
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                window.request_redraw();
            }
        }
    });

    possible_error.map_or(Ok(()),|e|Err(e))

}