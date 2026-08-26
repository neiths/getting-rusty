use crossterm::queue;
use crossterm::style::Print;
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{stdout, Error, Write};

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }

    pub fn print(text: &str) -> Result<(), Error> {
        queue!(stdout(), Print(text))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {height, width})
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}

