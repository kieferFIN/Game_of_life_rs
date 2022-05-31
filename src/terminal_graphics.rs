use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::QueueableCommand;
use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, DisableLineWrap, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, SetSize, size as terminal_size, size};

use crate::{Color as DataColor, ColoredDataType, Game, GError, RuleSet};

struct Ctx {
    orig_size: Option<(u16, u16)>,
}

impl Ctx {
    fn open(size: (u16, u16)) -> Result<Self, GError> {
        let orig_size = Some(terminal_size()?);
        enable_raw_mode()?;
        let mut out = stdout();
        out.queue(EnterAlternateScreen)?
            .queue(DisableLineWrap)?
            .queue(SetSize(size.0, size.1 ))?
            .queue(Hide)?
            .flush()?;
        Ok(Self { orig_size })
    }
    fn close(&mut self) -> Result<(), GError> {
        if let Some(size) = self.orig_size {
            let mut out = stdout();
            out.queue(LeaveAlternateScreen)?
                .queue(SetSize(size.0, size.1))?
                .queue(Show)?
                .queue(ResetColor)?
                .queue(EnableLineWrap)?
                .flush()?;
            disable_raw_mode()?;
        }

        Ok(())
    }
}

impl Drop for Ctx {
    fn drop(&mut self) {
        self.close().unwrap()
    }
}

pub fn run<R>(window_size: (u16, u16), game: &mut Game<R>) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let _ctx = Ctx::open(window_size)?;
    let mut is_playing = false;
    draw(&game)?;
    loop {
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char(' ') => is_playing ^= true,
                        KeyCode::Right if !is_playing => {
                            game.next_step();
                            draw(game)?
                        }
                        KeyCode::Char('c') => break,
                        _ => {}
                    }
                }
                Event::Resize(_, _) => draw(game)?,
                _ => {}
            }
        }

        if is_playing {
            game.next_step();
            draw(game)?;
        }
    }

    Ok(())
}

fn convert(c: DataColor) -> Color {
    Color::Rgb { r: c.0, g: c.1, b: c.2 }
}

fn draw<R>(game: &Game<R>) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let (_, h) = size()?;
    if h < game.grid.height {
        double_draw(game)
        //simple_draw(game)
    } else {
        simple_draw(game)
    }
}


fn double_draw<R>(game: &Game<R>) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let mut top_color = Color::Rgb { r: 0, g: 0, b: 0 };
    let mut bottom_color = Color::Rgb { r: 0, g: 0, b: 0 };
    let mut out = stdout();
    out.queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(SetBackgroundColor(top_color))?
        .flush()?;
    for y_half in 0..game.grid.height as i32 / 2 {
        for x in 0..game.grid.width as i32 {
            let tc = convert(game[(x, y_half * 2)].get_color());
            let bc = convert(game[(x, y_half * 2 + 1)].get_color());

            if top_color != tc {
                top_color = tc;
                out.queue(SetBackgroundColor(top_color))?;
            }
            if bottom_color != bc {
                bottom_color = bc;
                out.queue(SetForegroundColor(bottom_color))?;
            }
            out.queue(Print("▄"))?;
        }
        out.queue(MoveToNextLine(1))?;
    }
    out.queue(SetBackgroundColor(Color::Black))?.flush()?;
    Ok(())
}

fn simple_draw<R>(game: &Game<R>) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let mut current_color = Color::Rgb { r: 0, g: 0, b: 0 };
    let mut out = stdout();
    out.queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(SetBackgroundColor(current_color))?
        .flush()?;


    for (i, cell) in game {
        let color = convert(cell.get_color());
        if current_color != color {
            current_color = color;
            out.queue(SetBackgroundColor(current_color))?;
        }
        out.queue(Print(' '))?;
        if game.grid.width as i32 == i.0 + 1 {
            out.queue(MoveToNextLine(1))?;
        }
    }
    out.queue(SetBackgroundColor(Color::Black))?.flush()?;

    Ok(())
}