use field::{Direction, Field, Position, SnakeHeadField};
use rand::Rng;
use std::{
    collections::VecDeque,
    thread::sleep,
    time::{Duration, Instant},
};
pub mod field;

pub enum Input {
    ChangeDirection(Direction),
    Suicide,
    None,
}
pub trait Controller {
    fn get_input(&self) -> Input;
}

pub struct RenderInfo<'a> {
    pub map_size: (u32, u32),
    pub map: Vec<Field>,
    pub msg: &'a str,
}
pub trait Renderer {
    fn render_snake(&mut self, info: &RenderInfo);
}

pub struct Config {
    pub map_size: (u32, u32),
    pub start_tail: u32,
    pub snake_lag_ms: u128,
    pub snake_accel: f64,
}

pub struct State {
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

    fn handle_input(&mut self, input: &Input) {
        match input {
            Input::ChangeDirection(direction) => self.snake_head.change_direction(direction),
            Input::Suicide => self.snake_head.kill(),
            Input::None => (),
        };
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
                self.lag = (self.lag as f64 * (1. - config.snake_accel)) as u128;
            }
            _ => (),
        }
        if shrink {
            self.snake_tail.pop_front();
        }
    }

    fn flat_pos(pos: &Position, config: &Config) -> usize {
        (pos.y as u32 * config.map_size.0 + pos.x as u32) as usize
    }

    pub fn gen_info(&self, config: &Config) -> RenderInfo {
        //bounds should be checked here
        let size = config.map_size.0 * config.map_size.1;
        let mut res = [Field::Empty].repeat(size as usize);
        for pos in &self.snake_tail {
            res[Self::flat_pos(pos, config)] = Field::SnakeTail;
        }
        res[Self::flat_pos(&self.apple, config)] = Field::Apple;
        res[Self::flat_pos(self.snake_head.pos(), config)] = Field::SnakeHead;
        RenderInfo {
            map_size: config.map_size,
            map: res,
            msg: "",
        }
    }
}

const STAT_INERVAL_MS: u128 = 1000;

pub struct Game<'a, C: Controller, R: Renderer> {
    config: Config,
    controller: C,
    renderer: &'a mut R,
}
impl<'a, C: Controller, R: Renderer> Game<'a, C, R> {
    pub fn new(conf: Config, controller: C, renderer: &'a mut R) -> Self {
        Self {
            config: conf,
            controller,
            renderer,
        }
    }

    pub fn run(&mut self) -> u32 {
        let mut last_update = Instant::now();
        let mut last_collection = Instant::now();
        let mut state = State::new(&self.config);

        let mut fps_counter = 0;

        let mut msg = String::new();
        //TODO: Add 2 separate threads for render and logic
        while state.snake_head.is_alive() {
            state.handle_input(&self.controller.get_input());

            if last_update.elapsed().as_millis() > state.lag {
                state.update(&self.config);
                last_update = Instant::now();
            }

            let mut info = state.gen_info(&self.config);
            if last_collection.elapsed().as_millis() > STAT_INERVAL_MS {
                last_collection = Instant::now();
                msg = format!("FPS: {}", fps_counter);
                fps_counter = 0;
            }
            info.msg = &msg[..];
            self.renderer.render_snake(&info);

            fps_counter += 1;
            sleep(Duration::from_millis(0))
        }
        state.score
    }
}
