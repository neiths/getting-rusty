use std::io::{self, Write};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{size, disable_raw_mode, enable_raw_mode},
};

#[allow(dead_code)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn enable_raw_mode() -> Result<(), std::io::Error> {
        enable_raw_mode()
    }

    pub fn disable_raw_mode() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() {
        print!("\x1b[2J");
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(io::stdout(), MoveTo(x, y))
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
}
