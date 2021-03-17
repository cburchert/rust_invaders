use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Style, VideoMode};

use ::rust_invaders::Game;

fn main() {
    let window = RenderWindow::new(
        VideoMode::new(1800, 900, 32),
        "Rust Invaders",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut g = Game::new(window);
    g.run();
}
