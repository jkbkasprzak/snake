#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

pub struct SnakeHeadField {
    pos: Position,
    prev_pos: Position,
    direction: Direction,
    alive: bool,
}

impl SnakeHeadField {
    pub fn new(pos: Position) -> Self {
        SnakeHeadField {
            pos,
            prev_pos: pos,
            direction: Direction::Right,
            alive: true,
        }
    }
    fn looking_at(&self) -> Position {
        let mut looking_pos = self.pos;
        match self.direction {
            Direction::Left => looking_pos.x -= 1,
            Direction::Right => looking_pos.x += 1,
            Direction::Up => looking_pos.x += 1,
            Direction::Down => looking_pos.x -= 1,
        };
        looking_pos
    }
    fn fix_direction(&mut self) {
        if self.looking_at() == self.prev_pos {
            self.direction = self.direction.opposite();
        }
    }
    pub fn advance(&mut self) -> Position {
        self.fix_direction();
        self.prev_pos = self.pos;
        self.pos = self.looking_at();
        self.prev_pos
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn kill(&mut self) {
        self.alive = false;
    }
    pub fn pos(&self) -> &Position {
        &self.pos
    }
}

#[derive(PartialEq)]
pub enum Field {
    Invalid,
    Empty,
    SnakeHead,
    SnakeTail,
    Apple,
}
