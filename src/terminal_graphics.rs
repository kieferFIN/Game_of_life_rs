use std::fmt;
use std::io::{stdout, Stdout, StdoutLock, Write};
use std::time::Duration;

use crossterm::{Command, ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, DisableLineWrap, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, SetSize, size as terminal_size, size};

use crate::{Color as DataColor, ColoredDataType, Game, GError, RuleSet};

struct Ctx {
    orig_size: (u16, u16),
    out: Stdout,
    buffer: Vec<u8>
}

impl Ctx {
    fn open(size: (u16, u16), out: Stdout) -> Result<Self, GError> {
        let orig_size = terminal_size()?;
        enable_raw_mode()?;
        let buffer = Vec::with_capacity(200);
        let mut s = Self{orig_size,out, buffer};
        s.get_buffer().queue(EnterAlternateScreen)?
            .queue(DisableLineWrap)?
            .queue(SetSize(size.0, size.1 ))?
            .queue(Hide)?
            .flush()?;
        Ok(s)
    }
    fn close(&mut self) -> Result<(), GError> {
        let size = self.orig_size;
        self.get_buffer().queue(LeaveAlternateScreen)?
            .queue(SetSize(size.0, size.1))?
            .queue(Show)?
            .queue(ResetColor)?
            .queue(EnableLineWrap)?
            .flush()?;
        disable_raw_mode()?;

        Ok(())
    }

    fn get_buffer(&mut self) -> BufferWriter<StdoutLock>{
        BufferWriter::new(&mut self.buffer,self.out.lock())
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
    let mut ctx = Ctx::open(window_size, stdout())?;
    let mut is_playing = false;
    draw(game, ctx.get_buffer())?;
    loop {
        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char(' ') => is_playing ^= true,
                        KeyCode::Right if !is_playing => {
                            game.next_step();
                            draw(game,ctx.get_buffer())?
                        }
                        KeyCode::Char('c') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
                Event::Resize(_, _) => draw(game,ctx.get_buffer())?,
                _ => {}
            }
        }

        if is_playing {
            game.next_step();
            draw(game,ctx.get_buffer())?;
        }
    }

    Ok(())
}

fn convert(c: DataColor) -> Color {
    Color::Rgb { r: c.0, g: c.1, b: c.2 }
}

fn draw<R, W:Write>(game: &Game<R>, mut out: W) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let (_, h) = size()?;
    if h < game.grid.height {
        double_draw(game, out)
    } else {
        simple_draw(game,out)
    }
}


fn double_draw<R, W:Write>(game: &Game<R>, mut out: W) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let mut top_color = Color::Rgb { r: 0, g: 0, b: 0 };
    let mut bottom_color = Color::Rgb { r: 0, g: 0, b: 0 };
    out .queue(MoveTo(0, 0))?
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

fn simple_draw<R,W:Write>(game: &Game<R>, mut out: W) -> Result<(), GError>
    where R: RuleSet,
          R::Data: ColoredDataType {
    let mut current_color = Color::Rgb { r: 0, g: 0, b: 0 };
    out.queue(MoveTo(0, 0))?
        .queue(SetBackgroundColor(current_color))?
        .flush()?;


    for (i, cell) in game {
        let color = convert(cell.get_color());
        if current_color != color {
            current_color = color;
            out.queue(&SetBackgroundColor(current_color))?;
        }
        out.queue(Print(' '))?;
        if game.grid.width as i32 == i.0 + 1 {
            out.queue(MoveToNextLine(1))?;
        }
    }
    out.queue(SetBackgroundColor(Color::Black))?.flush()?;

    Ok(())
}

struct BufferWriter<'b,W:Write>{
    buffer: &'b mut Vec<u8>,
    out:W
}

impl<'b,W:Write> BufferWriter<'b,W> {
    fn new(buffer:&'b mut Vec<u8>, out: W)->Self{
        Self{buffer,out}
    }
}

impl<'b,W:Write> Write for BufferWriter<'b,W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.write_all(&self.buffer)?;
        self.buffer.clear();
        self.out.flush()

    }
}

impl<'b,W:Write> Drop for BufferWriter<'b,W> {
    fn drop(&mut self) {
        self.flush().unwrap()
    }
}