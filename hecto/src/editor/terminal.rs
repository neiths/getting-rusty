use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queued_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queued_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queued_command(MoveTo(position.col as u16, position.row as u16))
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queued_command(Hide)
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queued_command(Show)
    }

    pub fn print(text: &str) -> Result<(), Error> {
        Self::queued_command(Print(text))
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;

        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;

        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;

        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn queued_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command);;
        Ok(())
    }
}
