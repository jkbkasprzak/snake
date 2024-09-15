use rand::Rng;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct SnakeHead {
    pos: Vec2<i32>,
    prev_pos: Vec2<i32>,
    direction: Direction,
    alive: bool,
}

impl SnakeHead {
    pub fn new(pos: Vec2<i32>, prev_pos: Vec2<i32>, direction: Direction) -> Self {
        Self {
            pos,
            prev_pos,
            direction,
            alive: true,
        }
    }
    fn looking_at(&self) -> Vec2<i32> {
        let mut looking_pos = self.pos;
        match self.direction {
            Direction::Left => looking_pos.x -= 1,
            Direction::Right => looking_pos.x += 1,
            Direction::Up => looking_pos.y += 1,
            Direction::Down => looking_pos.y -= 1,
        };
        looking_pos
    }
    fn fix_direction(&mut self) {
        if self.looking_at() == self.prev_pos {
            self.direction = self.direction.opposite();
        }
    }
    pub fn advance(&mut self) -> Vec2<i32> {
        self.fix_direction();
        self.prev_pos = self.pos;
        self.pos = self.looking_at();
        self.prev_pos
    }
    pub fn change_direction(&mut self, new_direction: &Direction) {
        self.direction = *new_direction;
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn kill(&mut self) {
        self.alive = false;
    }
    pub fn pos(&self) -> &Vec2<i32> {
        &self.pos
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Config {
    pub map_size: Vec2<u32>,
    pub start_tail: u32,
    pub step_interval: Duration,
    pub step_accel: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Input {
    ChangeDirection(Direction),
    Suicide,
    None,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Field {
    Invalid,
    Empty,
    SnakeHead,
    SnakeTail,
    Apple,
}

#[derive(Clone)]
pub struct Map {
    shape: Vec2<u32>,
    fields: Vec<Field>,
}

impl From<&State> for Map {
    fn from(state: &State) -> Self {
        Map {
            shape: state.config.map_size,
            fields: Self::gen_fields(state),
        }
    }
}

impl Map {
    fn pos_offset(pos: &Vec2<i32>, buf_shape: &Vec2<u32>) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.x as u32 >= buf_shape.x || pos.y as u32 >= buf_shape.y {
            return None;
        }
        Some((pos.y as u32 * buf_shape.x + pos.x as u32) as usize)
    }

    fn gen_fields(state: &State) -> Vec<Field> {
        let map_shape = &state.config.map_size;
        let size = map_shape.x * map_shape.y;
        let mut fields = [Field::Empty].repeat(size as usize);
        for pos in &state.snake_tail {
            if let Some(offset) = Self::pos_offset(pos, map_shape) {
                fields[offset] = Field::SnakeTail;
            }
        }
        if let Some(offset) = Self::pos_offset(&state.apple, map_shape) {
            fields[offset] = Field::Apple;
        }
        if let Some(offset) = Self::pos_offset(state.snake_head.pos(), map_shape) {
            fields[offset] = Field::SnakeHead;
        }
        fields
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
    pub fn shape(&self) -> &Vec2<u32> {
        &self.shape
    }
}

#[derive(Clone)]
pub struct State {
    config: Config,
    score: u32,
    snake_head: SnakeHead,
    snake_tail: VecDeque<Vec2<i32>>,
    apple: Vec2<i32>,
    step_interval: Duration,
    last_step_interval: Duration,
    last_update: Instant,
    started: bool,
}

impl State {
    pub fn new(config: Config) -> State {
        let mid = Vec2::new(
            (config.map_size.x / 2) as i32,
            (config.map_size.y / 2) as i32,
        );
        let mut tail = VecDeque::new();
        for i in 0..config.start_tail {
            tail.push_front(Vec2::new(mid.x - i as i32 - 1, mid.y));
        }
        let mut new_state = State {
            config,
            score: 0,
            snake_head: SnakeHead::new(mid, *tail.back().unwrap(), Direction::Right),
            snake_tail: tail,
            apple: mid,
            step_interval: config.step_interval,
            last_step_interval: Duration::ZERO,
            last_update: Instant::now(),
            started: false,
        };
        new_state.apple = new_state.rand_empty_pos();
        new_state
    }

    fn rand_pos(&self) -> Vec2<i32> {
        Vec2::new(
            rand::thread_rng().gen_range(0..self.config.map_size.x) as i32,
            rand::thread_rng().gen_range(0..self.config.map_size.y) as i32,
        )
    }

    fn rand_empty_pos(&self) -> Vec2<i32> {
        let mut rand_pos = self.rand_pos();
        while self.check_pos(&rand_pos) != Field::Empty {
            rand_pos = self.rand_pos();
        }
        rand_pos
    }

    fn check_pos(&self, pos: &Vec2<i32>) -> Field {
        if pos.x < 0
            || pos.y < 0
            || pos.x as u32 >= self.config.map_size.x
            || pos.y as u32 >= self.config.map_size.y
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

    pub fn handle_input(&mut self, input: &Input) {
        if !self.started && *input != Input::None {
            self.started = true;
        }
        match input {
            Input::ChangeDirection(direction) => self.snake_head.change_direction(direction),
            Input::Suicide => self.snake_head.kill(),
            Input::None => (),
        };
    }

    pub fn update(&mut self) {
        let elapsed: Duration = self.last_update.elapsed();
        if !self.started || elapsed < self.step_interval {
            return;
        }
        self.last_step_interval = elapsed;
        self.last_update = Instant::now();
        self.snake_tail.push_back(self.snake_head.advance());

        let mut shrink = true;
        match self.check_pos(self.snake_head.pos()) {
            Field::Invalid => self.snake_head.kill(),
            Field::SnakeTail => self.snake_head.kill(),
            Field::Apple => {
                shrink = false;
                self.apple = self.rand_empty_pos();
                self.score += 1;
                self.step_interval = self.step_interval.mul_f64(1. - self.config.step_accel)
            }
            _ => (),
        }
        if shrink {
            self.snake_tail.pop_front();
        }
    }

    pub fn step_interval(&self) -> Duration {
        self.step_interval
    }
    pub fn real_step_interval(&self) -> Duration {
        self.last_step_interval
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn is_terminal(&self) -> bool {
        !self.snake_head.is_alive()
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}
