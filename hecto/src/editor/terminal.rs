use crossterm::{queue, Command};
use crossterm::style::Print;
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{stdout, Error, Write};
use core::fmt::Display;

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
        Self::queued_command(Clear(ClearType::All))
    }
    
    pub fn clear_line() -> Result<(), Error> {
        Self::queued_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queued_command(MoveTo(position.x, position.y))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queued_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queued_command(Show)
    }

    pub fn print<T: Display>(text: T) -> Result<(), Error> {
        Self::queued_command(Print(text))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {height, width})
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn queued_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command);
        Ok(())
    }
}

