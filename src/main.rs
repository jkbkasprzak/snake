use snake::{Config, Game};

fn main() {
    let cfg = Config {
        map_size: (32, 32),
        start_tail: 2,
        snake_lag_ms: 1000,
        snake_accel_ms: 20,
    };
    let game = Game::new(cfg);
    let score = game.run();
    println!("You scored {score}!");
}
