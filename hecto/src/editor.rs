use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
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
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            if current_row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                Terminal::print("~")?;
            }
            if current_row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);

        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        let Size{height, ..}= Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows(&self)?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
