use core::str;
use crossterm::event::{self, poll, KeyCode};
use crossterm::style::{self};
use crossterm::terminal::{self, BeginSynchronizedUpdate, EndSynchronizedUpdate};
use crossterm::{cursor, execute};
use snake::field::Direction;
use snake::{Config, Controller, Game, Input, Renderer};
use std::io::stdout;
use std::time::Duration;

struct TerminalController {}

impl Controller for TerminalController {
    fn get_input(&self) -> Input {
        if poll(Duration::from_millis(10)).unwrap_or(false) {
            if let Ok(event) = event::read() {
                match event {
                    event::Event::Key(key_event) => {
                        return match key_event.code {
                            KeyCode::Char('w') => Input::ChangeDirection(Direction::Up),
                            KeyCode::Char('s') => Input::ChangeDirection(Direction::Down),
                            KeyCode::Char('a') => Input::ChangeDirection(Direction::Left),
                            KeyCode::Char('d') => Input::ChangeDirection(Direction::Right),
                            KeyCode::Esc => Input::Suicide,
                            _ => Input::None,
                        }
                    }
                    _ => (),
                }
            }
        }
        Input::None
    }
}

struct TerminalRenderer {
    size: (u16, u16),
}

impl TerminalRenderer {
    fn new(cfg: &Config) -> Self {
        terminal::enable_raw_mode();
        let size = ((cfg.map_size.0 + 2) as u16, (cfg.map_size.1 + 2) as u16);
        terminal::SetSize(size.0, size.1);
        TerminalRenderer { size }
    }
}

impl Renderer for TerminalRenderer {
    fn render(&self, info: &snake::RenderInfo) {
        let flat_size = self.size.0 * self.size.1;
        let mut buffer = Vec::from(['%' as u8]).repeat(flat_size.into());
        let mut offset = (self.size.0 + 1) as usize;

        for (i, field) in info.map.iter().enumerate() {
            buffer[i + offset] = match field {
                snake::field::Field::Invalid => '!',
                snake::field::Field::Empty => ' ',
                snake::field::Field::SnakeHead => 'O',
                snake::field::Field::SnakeTail => 'O',
                snake::field::Field::Apple => '@',
            } as u8;
            if (i + 1) % (info.map_size.0 as usize) == 0 {
                offset += 2;
            }
        }
        let display_buffer = str::from_utf8(&buffer).expect("TO MUSI DZIALAC");
        let mut stdout = stdout();
        execute!(stdout, BeginSynchronizedUpdate);
        execute!(stdout, terminal::Clear(terminal::ClearType::All));
        execute!(stdout, cursor::MoveTo(0, self.size.1));
        let mut pos = 0;
        while pos < buffer.len() {
            execute!(
                stdout,
                style::Print(&display_buffer[pos..pos + self.size.0 as usize])
            );
            execute!(stdout, cursor::MoveToPreviousLine(1));
            pos += self.size.0 as usize;
        }
        execute!(stdout, EndSynchronizedUpdate);
    }
}

fn main() {
    let cfg = Config {
        map_size: (16, 16),
        start_tail: 2,
        snake_lag_ms: 200,
        snake_accel_ms: 20,
    };
    let renderer = TerminalRenderer::new(&cfg);
    let game = Game::new(cfg, TerminalController {}, renderer);
    let score = game.run();
    println!("You scored {score}!");
}
