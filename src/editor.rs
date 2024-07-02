use crossterm::cursor::{position, MoveTo};
use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
    cursor_x: u16,
    cursor_y: u16,
}

impl Editor {
    pub fn constructor() -> Self {
        Editor {
            should_quit: false,
            cursor_x: 0,
            cursor_y: 0,
        }
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        Self::draw_rows();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        // let row = position().x;
        // let column = 0;
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            self.set_position();
            match code {
                // Char('b') if *modifiers == KeyModifiers::CONTROL => {
                //     self.should_quit = true;
                // }
                KeyCode::Up => {
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                    }
                    execute!(std::io::stdout(), MoveTo(self.cursor_y, self.cursor_x)).unwrap();
                }
                KeyCode::Down => {
                    if self.cursor_y > 0 {
                        self.cursor_y += 1;
                    }
                    execute!(std::io::stdout(), MoveTo(self.cursor_y, self.cursor_x)).unwrap();
                }
                KeyCode::Left => {
                    if self.cursor_x > 0 {
                        self.cursor_x -= 1;
                    }
                    execute!(std::io::stdout(), MoveTo(self.cursor_y, self.cursor_x)).unwrap();
                }
                KeyCode::Right => {
                    if self.cursor_x > 0 {
                        self.cursor_x += 1;
                    }
                    execute!(std::io::stdout(), MoveTo(self.cursor_y, self.cursor_x)).unwrap();
                }
                _ => {
                    println!("x:{} y:{}", self.cursor_x, self.cursor_y);
                }
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
    fn draw_rows() {
        for i in 0..5 {
            println!("{} ", '~');
        }
    }
    fn read_position() {
        match position() {
            Ok((x, y)) => {
                println!("The current cursor position is (x: {}, y: {}).", x, y);
            }
            Err(e) => {
                eprintln!("Failed to get cursor position: {}", e);
            }
        }
    }
    fn set_position(&mut self) {
        match position() {
            Ok((x, y)) => {
                self.cursor_x = x;
                self.cursor_y = y;
            }
            Err(e) => {
                eprintln!("Failed to get cursor position: {}", e);
            }
        }
    }
}
