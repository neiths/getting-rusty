use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message(self)?;
            } else {
                Self::draw_empty_rows(self)?;
            }

            if current_row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message(&self) -> Result<(), Error> {
        let welcome_message = format!("{} -- version {}", NAME, VERSION);
        let mut padding = (Terminal::size()?.width as usize - welcome_message.len()) / 2;
        if padding > 0 {
            Terminal::print("~")?;
            padding -= 1;
        }
        for _ in 0..padding {
            Terminal::print(" ")?;
        }
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_rows(&self) -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
