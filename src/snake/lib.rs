pub mod logic;

pub use logic::{Config, Direction, Input, Map, State, Vec2};
use std::{thread::sleep, time::Duration};

pub trait Controller {
    fn get_input(&self) -> Input;
}
pub trait Renderer {
    fn render_snake(&mut self, state: &State);
}

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
        let mut state = State::new(self.config);
        //TODO: Add 2 separate threads for render and logic
        while !state.is_terminal() {
            state.handle_input(&self.controller.get_input());
            state.update();
            self.renderer.render_snake(&state);
            sleep(Duration::from_millis(0))
        }
        state.score()
    }
}
