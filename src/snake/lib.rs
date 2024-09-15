pub mod logic;

use logic::State;
pub use logic::{Config, Direction, Input, RenderInfo, Vec2};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub trait Controller {
    fn get_input(&self) -> Input;
}
pub trait Renderer {
    fn render_snake(&mut self, info: &RenderInfo);
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
        let mut last_collection = Instant::now();
        let mut state = State::new(self.config);

        let mut fps_counter = 0;

        let mut msg = String::new();
        //TODO: Add 2 separate threads for render and logic
        while !state.is_terminal() {
            state.handle_input(&self.controller.get_input());
            state.update();

            let mut info = state.render_info();
            if last_collection.elapsed().as_millis() > STAT_INERVAL_MS {
                last_collection = Instant::now();
                msg = format!("FPS: {}", fps_counter);
                fps_counter = 0;
            }
            info.message = format!("{} {}", info.message, msg);
            self.renderer.render_snake(&info);

            fps_counter += 1;
            sleep(Duration::from_millis(0))
        }
        state.score()
    }
}
