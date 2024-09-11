use field::{Field, Position, SnakeHeadField};
use rand::Rng;
use std::{
    collections::VecDeque,
    thread::sleep,
    time::{Duration, Instant},
};

pub mod field;

pub struct Config {
    pub map_size: (u32, u32),
    pub start_tail: u32,
    pub snake_lag_ms: u128,
    pub snake_accel_ms: u128,
}

struct State {
    score: u32,
    snake_head: SnakeHeadField,
    snake_tail: VecDeque<Position>,
    apple: Position,
    lag: u128,
}
impl State {
    fn rand_pos(config: &Config) -> Position {
        Position::new(
            rand::thread_rng().gen_range(0..config.map_size.0) as i32,
            rand::thread_rng().gen_range(0..config.map_size.1) as i32,
        )
    }
    fn new(config: &Config) -> State {
        let mid = Position::new(
            (config.map_size.0 / 2) as i32,
            (config.map_size.1 / 2) as i32,
        );
        let mut tail = VecDeque::new();
        for i in 0..config.start_tail {
            tail.push_back(Position::new(mid.x - i as i32 - 1, mid.y));
        }
        let mut new_state = State {
            score: 0,
            snake_head: SnakeHeadField::new(mid),
            snake_tail: tail,
            apple: mid,
            lag: config.snake_lag_ms,
        };
        new_state.apple = new_state.rand_empty_pos(config);
        new_state
    }

    fn rand_empty_pos(&self, config: &Config) -> Position {
        let mut rand_pos = Self::rand_pos(config);
        while self.check_pos(&rand_pos, config) != Field::Empty {
            rand_pos = Self::rand_pos(config);
        }
        rand_pos
    }

    fn check_pos(&self, pos: &Position, config: &Config) -> Field {
        if pos.x < 0
            || pos.y < 0
            || pos.x >= config.map_size.0 as i32
            || pos.y >= config.map_size.1 as i32
        {
            return Field::Invalid;
        }
        for tail in &self.snake_tail {
            if *pos == *tail {
                return Field::SnakeTail;
            }
        }
        if *pos == self.apple {
            return Field::Apple;
        }
        if *pos == *self.snake_head.pos() {
            return Field::SnakeHead;
        }
        Field::Empty
    }

    fn update(&mut self, config: &Config) {
        self.snake_tail.push_back(self.snake_head.advance());

        let mut shrink = true;
        match self.check_pos(self.snake_head.pos(), config) {
            Field::Invalid => self.snake_head.kill(),
            Field::SnakeTail => self.snake_head.kill(),
            Field::Apple => {
                shrink = false;
                self.apple = self.rand_empty_pos(config);
                self.score += 1;
                self.lag -= config.snake_accel_ms;
            }
            _ => (),
        }
        if shrink {
            self.snake_tail.pop_front();
        }
    }
}
pub struct Game {
    config: Config,
}
impl Game {
    pub fn new(conf: Config) -> Self {
        Self { config: conf }
    }

    pub fn run(&self) -> u32 {
        let mut last_update = Instant::now();
        let mut state = State::new(&self.config);
        while state.snake_head.is_alive() {
            let loop_start = Instant::now();
            // todo handle input
            if last_update.elapsed().as_millis() > state.lag {
                state.update(&self.config);
                last_update = Instant::now();
            }
            // todo render
            // max 200 iter/s
            let sleep_time = 5u64.saturating_sub(loop_start.elapsed().as_millis() as u64);
            sleep(Duration::from_millis(sleep_time))
        }
        state.score
    }
}
