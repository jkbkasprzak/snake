use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use crossterm::style::{self, Color};
use crossterm::terminal::{self, BeginSynchronizedUpdate, EndSynchronizedUpdate};
use crossterm::{cursor, execute};
use snake::logic::{Direction, Vec2};
use snake::{Controller, Input, Renderer};
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

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

pub struct TerminalDisplay {
    stdout: Stdout,
    size: Vec2<u32>,
    offset: Vec2<u32>,
    last_render: Instant,
    last_text_update: Instant,
}

impl TerminalDisplay {
    const WIDTH: usize = 2;
    const TEXT_UPDATE_INTERVAL: Duration = Duration::from_millis(1_000);
    pub fn new(size: Vec2<u32>, offset: Vec2<u32>) -> Self {
        TerminalDisplay {
            stdout: stdout(),
            size,
            offset,
            last_render: Instant::now(),
            last_text_update: Instant::now(),
        }
    }

    fn print_block(&mut self, color: style::Color) {
        let block = "\u{2588}".repeat(Self::WIDTH);
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
        let full_line = "\u{2588}".repeat(Self::WIDTH * self.size.x as usize);
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
            style::Print("\n"),
            style::SetForegroundColor(style::Color::White),
            cursor::Show
        )
        .unwrap();
    }

    fn print_snake_map(&mut self, map: &snake::Map) {
        execute!(
            self.stdout,
            BeginSynchronizedUpdate,
            cursor::MoveToPreviousLine(self.offset.y as u16),
            cursor::MoveRight(self.offset.x as u16 * Self::WIDTH as u16)
        )
        .unwrap();

        for (i, field) in map.fields().iter().enumerate() {
            match field {
                snake::logic::Field::Invalid => self.print_block(Color::Magenta),
                snake::logic::Field::Empty => self.print_block(Color::Black),
                snake::logic::Field::SnakeHead => self.print_block(Color::DarkGreen),
                snake::logic::Field::SnakeTail => self.print_block(Color::DarkGreen),
                snake::logic::Field::Apple => self.print_block(Color::Red),
            }
            if (i + 1) % map.shape().x as usize == 0 {
                execute!(
                    self.stdout,
                    cursor::MoveToPreviousLine(1),
                    cursor::MoveRight(self.offset.x as u16 * Self::WIDTH as u16)
                )
                .unwrap();
            }
        }
        execute!(
            self.stdout,
            cursor::MoveToNextLine((map.shape().y + self.offset.y) as u16),
            EndSynchronizedUpdate
        )
        .unwrap();
    }
}

impl Renderer for TerminalDisplay {
    fn render_snake(&mut self, state: &snake::State) {
        self.print_snake_map(&snake::Map::from(state));
        let fps: f64 = 1000. / self.last_render.elapsed().as_millis() as f64;
        self.last_render = Instant::now();

        if self.last_text_update.elapsed() >= Self::TEXT_UPDATE_INTERVAL {
            self.last_text_update = Instant::now();
            let mut msg = String::new();
            if !state.started() {
                msg = "Controls: W/S/A/D/<ESC>".to_string();
            }
            let steps_per_s = state.step_interval().as_millis();
            let real_steps_per_s = state.real_step_interval().as_millis();
            msg += &format!(
                " [FPS: {fps:.0}, DELAY: {steps_per_s}ms, REAL_DELAY: {real_steps_per_s}ms]"
            );
            self.print_text(&msg);
        }
    }
}
