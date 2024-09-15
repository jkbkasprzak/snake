mod terminal;

use std::time::Duration;

use snake::{Config, Game, Vec2};
use terminal::{TerminalController, TerminalDisplay};

fn main() {
    let cfg = Config {
        map_size: Vec2::new(32, 16),
        start_tail: 2,
        step_interval: Duration::from_millis(100),
        step_accel: 0.05,
    };
    let mut display = TerminalDisplay::new(
        Vec2::new(cfg.map_size.x + 2, cfg.map_size.y + 2),
        Vec2::new(1, 1),
    );
    display.prepare();
    let mut game = Game::new(cfg, TerminalController, &mut display);
    let score = game.run();
    display.restore();
    println!("\nYou scored {score}!");
}
