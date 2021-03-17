use sfml::graphics::RenderWindow;
use sfml::system::Clock;
use sfml::window::{Event, Key};

use crate::player::Player;

pub struct Game {
	window: RenderWindow,
	player: Player,
}

impl Game {
	pub fn new(window: RenderWindow) -> Game {
		Game {
			window,
			player: Player::new(),
		}
	}

	pub fn run(&mut self) {
		let mut clock = Clock::start();
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				self.handle_event(event);
			}
			self.advance_time(clock.restart().as_seconds());
			self.render();
		}
	}

	fn handle_event(&mut self, event: Event) {
		match event {
			Event::Closed => self.window.close(),
			Event::KeyPressed { code, .. } => self.handle_key_pressed(code),
			_ => (),
		}
	}

	fn handle_key_pressed(&mut self, key: Key) {
		match key {
			Key::Escape => self.window.close(),
			_ => (),
		}
	}

	fn advance_time(&mut self, time_delta_s: f32) {}

	fn render(&mut self) {
		self.player.render(&mut self.window);
		self.window.display();
	}
}
