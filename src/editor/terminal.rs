use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, terminal};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    pub fn draw_rows() -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(0, 0))?;
        let height = terminal::size()?.1;
        for _ in 1..height {
            println!("{} ", '~');
        }
        execute!(stdout(), MoveTo(1, 0))
    }
}
