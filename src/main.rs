mod terminal;

use snake::{Config, Game};
use terminal::{TerminalController, TerminalDisplay, Vec2};

fn main() {
    let cfg = Config {
        map_size: (32, 16),
        start_tail: 2,
        snake_lag_ms: 100,
        snake_accel: 0.05,
    };
    let mut display = TerminalDisplay::new(
        Vec2::new(cfg.map_size.0 as i32 + 2, cfg.map_size.1 as i32 + 2),
        Vec2::new(1, 1),
    );
    display.prepare();
    let mut game = Game::new(cfg, TerminalController, &mut display);
    let score = game.run();
    display.restore();
    println!("\nYou scored {score}!");
}
