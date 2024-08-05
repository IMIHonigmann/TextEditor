use crossterm::cursor::{position, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::size;
use crossterm::{queue, Command};
use std::cmp::max;
use std::io::stdout;
use std::io::Write;
mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    cursor_x: u16,
    cursor_y: u16,
    width: u16,
    height: u16,
}

impl Editor {
    pub fn constructor() -> Self {
        Editor {
            should_quit: false,
            cursor_x: 0,
            cursor_y: 0,
            width: 0,
            height: 0,
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
            stdout().flush().unwrap();
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            self.set_position();
            queue!(stdout(), Hide).unwrap();
            match code {
                KeyCode::Up => {
                    queue!(stdout(), MoveUp(1)).unwrap();
                }
                KeyCode::Down => {
                    queue!(stdout(), MoveDown(1)).unwrap();
                }
                KeyCode::Left => {
                    queue!(stdout(), MoveLeft(1)).unwrap();
                }
                KeyCode::Right => {
                    queue!(stdout(), MoveRight(1)).unwrap();
                }
                KeyCode::Backspace => {
                    queue!(stdout(), Print(" ")).unwrap();
                    queue!(stdout(), MoveLeft(2)).unwrap();
                    self.set_position();
                }
                KeyCode::Enter => {
                    queue!(stdout(), MoveDown(1)).unwrap();
                    queue!(stdout(), MoveTo(0, self.cursor_y + 1)).unwrap();
                }
                Char('l') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char('m') => {
                    self.set_position();
                    Editor::read_position();
                    self.set_terminal_size_parameters();
                    let output = format!(" width: {} height: {}", self.width, self.height);
                    queue!(stdout(), Print(output)).unwrap();
                }
                _ => {}
            }
            stdout().flush().unwrap();
            self.set_position();

            self.vimlike_tildas();
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        queue!(stdout(), Show).unwrap();
        if self.should_quit {
            Terminal::clear_screen()?;
            queue!(stdout(), Print("Goodbye.\r\n")).unwrap();
        }
        Ok(())
    }
    fn vimlike_tildas(&mut self) {
        queue!(stdout(), MoveTo(0, self.cursor_y)).unwrap();
        queue!(stdout(), Print("~")).unwrap();
        queue!(stdout(), MoveTo(max(1, self.cursor_x), self.cursor_y)).unwrap();
    }
    fn read_position() {
        match position() {
            Ok((x, y)) => {
                let formatted_output_position =
                    format!("The current cursor position is (x: {}, y: {}).\r\n", x, y);
                queue!(stdout(), Print(formatted_output_position)).unwrap();
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
    fn set_terminal_size_parameters(&mut self) {
        match size() {
            Ok((w, h)) => {
                self.width = w;
                self.height = h;
            }
            Err(e) => {
                eprintln!("Failed to set terminal size parameters: {}", e);
                self.width = 0;
                self.height = 0;
            }
        }
    }
    fn get_terminal_size(&mut self) -> (u16, u16) {
        match size() {
            Ok((w, h)) => (w, h),
            Err(_) => (0, 0),
        }
    }
}
