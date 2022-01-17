use std::time::Instant;
use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Sprite, Texture, Text, View};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, Key, Style};
use crate::{ColoredDataType, Game, RuleSet};

pub fn run<R>(window_size: (u32, u32), game: &mut Game<R>)
    where R: RuleSet,
          R::Data: ColoredDataType {
    let width = game.grid.width;
    let height = game.grid.height;
    let area: Vector2f = (width as f32, height as f32).into();

    let font = Font::from_file("sansation.ttf").unwrap();
    let mut fps_text = Text::default();
    fps_text.set_font(&font);
    let ctx_settings = ContextSettings::default();
    let mut window = RenderWindow::new(window_size, "GOL", Style::CLOSE, &ctx_settings);
    window.set_framerate_limit(60);
    let mut texture = Texture::new(width as u32, height as u32).unwrap();
    let view = View::new(area * 0.5, area);
    window.set_view(&view);
    let mut is_playing = false;
    let mut show_fps = false;
    let mut prev_time = Instant::now();
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::SPACE, .. } => is_playing ^= true,
                Event::KeyPressed { code: Key::F, .. } => show_fps ^= true,
                _ => ()
            }
        }
        if is_playing {
            game.next_step();
        }
        let curr_time = Instant::now();
        let fps = 1.0 / (curr_time - prev_time).as_secs_f32();
        prev_time = curr_time;
        unsafe {
            texture.update_from_pixels(&*game.to_raw_colors(), game.grid.width as u32, game.grid.height as u32, 0, 0);
        }
        let sprite = Sprite::with_texture(&texture);

        window.clear(Color::BLACK);
        window.draw(&sprite);
        if show_fps {
            fps_text.set_string(&fps.to_string());
            window.draw(&fps_text);
        }
        window.display();
    }
}