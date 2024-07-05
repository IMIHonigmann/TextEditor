use crossterm::cursor::{position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent};
use crossterm::execute;
use std::cmp::max;
use std::io::stdout;
mod terminal;
use terminal::Terminal;

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
        Terminal::initialize().unwrap();
        Terminal::draw_rows().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
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
        if let Key(KeyEvent { code, .. }) = event {
            self.set_position();
            match code {
                KeyCode::Up => {
                    execute!(stdout(), MoveUp(1)).unwrap();
                }
                KeyCode::Down => {
                    execute!(stdout(), MoveDown(1)).unwrap();
                }
                KeyCode::Left => {
                    execute!(stdout(), MoveLeft(1)).unwrap();
                }
                KeyCode::Right => {
                    execute!(stdout(), MoveRight(1)).unwrap();
                }
                KeyCode::Backspace => {
                    print!(" ");
                    execute!(stdout(), MoveLeft(2)).unwrap();
                    self.set_position();
                    if self.cursor_x == 0 {
                        execute!(stdout(), MoveUp(1)).unwrap();
                    }
                }
                KeyCode::Enter => {
                    execute!(stdout(), MoveDown(1)).unwrap();
                    execute!(stdout(), MoveTo(0, self.cursor_y + 1)).unwrap();
                    print!("~");
                    execute!(stdout(), MoveRight(1)).unwrap();
                }
                Char('m') => {
                    self.set_position();
                    Editor::read_position();
                }
                _ => {
                    println!("x:{} y:{}", self.cursor_x, self.cursor_y);
                }
            }
            self.set_position();
            execute!(stdout(), MoveTo(max(1, self.cursor_x), self.cursor_y)).unwrap();
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
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
                self.cursor_x = max(0, x);
                self.cursor_y = max(0, y);
            }
            Err(e) => {
                eprintln!("Failed to get cursor position: {}", e);
            }
        }
    }
}
