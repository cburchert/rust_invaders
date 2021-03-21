use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Style, VideoMode};

use ::rust_invaders::{Game, SCREEN_H, SCREEN_W};

fn main() {
    let window = RenderWindow::new(
        VideoMode::new(SCREEN_W, SCREEN_H, 32),
        "Rust Invaders",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut g = Game::new(window);
    g.run();
}
