use crate::terminal::Terminal;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use std::io;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        if let Err(error) = Terminal::enable_raw_mode() {
            die(&error);
        }

        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }

        let _ = Terminal::disable_raw_mode();
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::clear_screen();
        Terminal::move_cursor_to(0, 0)?;
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = read()?;
        if let Event::Key(key_event) = pressed_key {
            match key_event.code {
                KeyCode::Char('q') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for _ in 0..height {
            println!("~\r");
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

fn die(e: &io::Error) {
    Terminal::clear_screen();
    let _ = Terminal::disable_raw_mode();
    panic!("{}", e);
}
