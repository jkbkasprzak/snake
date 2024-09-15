use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use crossterm::style::{self, Color};
use crossterm::terminal::{self, BeginSynchronizedUpdate, EndSynchronizedUpdate};
use crossterm::{cursor, execute};
use snake::logic::{Direction, Vec2};
use snake::{Controller, Input, Renderer};
use std::io::{stdout, Stdout};
use std::time::Duration;

pub struct TerminalController;

impl Controller for TerminalController {
    fn get_input(&self) -> Input {
        let mut key_event: Option<KeyEvent> = None;
        while poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(event)) = event::read() {
                key_event = Some(event)
            }
        }
        if let Some(event) = key_event {
            match event.code {
                KeyCode::Char('w') => Input::ChangeDirection(Direction::Up),
                KeyCode::Char('s') => Input::ChangeDirection(Direction::Down),
                KeyCode::Char('a') => Input::ChangeDirection(Direction::Left),
                KeyCode::Char('d') => Input::ChangeDirection(Direction::Right),
                KeyCode::Esc => Input::Suicide,
                _ => Input::None,
            }
        } else {
            Input::None
        }
    }
}

const WIDTH: usize = 2;
pub struct TerminalDisplay {
    stdout: Stdout,
    size: Vec2<u32>,
    offset: Vec2<u32>,
}

impl TerminalDisplay {
    pub fn new(size: Vec2<u32>, offset: Vec2<u32>) -> Self {
        TerminalDisplay {
            stdout: stdout(),
            size,
            offset,
        }
    }

    fn print_block(&mut self, color: style::Color) {
        let block = "\u{2588}".repeat(WIDTH);
        execute!(
            self.stdout,
            style::SetForegroundColor(color),
            style::Print(&block)
        )
        .unwrap();
    }
    fn print_text(&mut self, text: &str) {
        execute!(
            self.stdout,
            cursor::MoveToNextLine(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::SetForegroundColor(style::Color::Yellow),
            style::Print(text),
            cursor::MoveToPreviousLine(1)
        )
        .unwrap();
    }

    pub fn prepare(&mut self) {
        terminal::enable_raw_mode().unwrap();
        let screen_space = "\n".repeat(self.size.y as usize);
        let full_line = "\u{2588}".repeat(WIDTH * self.size.x as usize);
        execute!(
            self.stdout,
            style::SetForegroundColor(style::Color::DarkGrey),
            cursor::Hide,
            style::Print(&screen_space),
            cursor::MoveToPreviousLine(self.size.y as u16),
        )
        .unwrap();
        for _i in 0..self.size.y {
            execute!(
                self.stdout,
                style::Print(&full_line),
                cursor::MoveToNextLine(1),
            )
            .unwrap();
        }
        //move back to the last line of the screen
        execute!(self.stdout, cursor::MoveToPreviousLine(1),).unwrap();
    }

    pub fn restore(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(
            self.stdout,
            style::SetForegroundColor(style::Color::White),
            cursor::Show
        )
        .unwrap();
    }
}

impl Renderer for TerminalDisplay {
    fn render_snake(&mut self, info: &snake::RenderInfo) {
        execute!(
            self.stdout,
            BeginSynchronizedUpdate,
            cursor::MoveToPreviousLine(self.offset.y as u16),
            cursor::MoveRight(self.offset.x as u16 * WIDTH as u16)
        )
        .unwrap();

        for (i, field) in info.map.fields().iter().enumerate() {
            match field {
                snake::logic::Field::Invalid => self.print_block(Color::Magenta),
                snake::logic::Field::Empty => self.print_block(Color::Black),
                snake::logic::Field::SnakeHead => self.print_block(Color::DarkGreen),
                snake::logic::Field::SnakeTail => self.print_block(Color::DarkGreen),
                snake::logic::Field::Apple => self.print_block(Color::Red),
            }
            if (i + 1) % info.map.shape().x as usize == 0 {
                execute!(
                    self.stdout,
                    cursor::MoveToPreviousLine(1),
                    cursor::MoveRight(self.offset.x as u16 * WIDTH as u16)
                )
                .unwrap();
            }
        }
        execute!(
            self.stdout,
            cursor::MoveToNextLine((info.map.shape().y + self.offset.y) as u16),
            EndSynchronizedUpdate
        )
        .unwrap();
        self.print_text(&info.message);
    }
}
