use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Clock;
use sfml::window::{Event, Key};

use crate::alien::AlienGroup;
use crate::bullet::Bullet;
use crate::player::Player;
use crate::settings::PLAYER_SPEED;

pub struct Game {
	window: RenderWindow,
	player: Player,
	bullets: Vec<Bullet>,
	aliens: AlienGroup,
}

impl Game {
	pub fn new(window: RenderWindow) -> Game {
		Game {
			window,
			player: Player::new(),
			bullets: vec![],
			aliens: AlienGroup::new(10, 3),
		}
	}

	pub fn run(&mut self) {
		let mut clock = Clock::start();
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				self.handle_event(event);
			}
			self.advance_time(clock.restart().as_seconds());

			self.window.clear(Color::BLACK);
			self.render();
			self.window.display();
		}
	}

	pub fn add_bullet(&mut self, bullet: Bullet) {
		self.bullets.push(bullet)
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

	fn advance_time(&mut self, time_delta_s: f32) {
		if Key::A.is_pressed() {
			self.player.move_horizontally(-PLAYER_SPEED * time_delta_s);
		}
		if Key::D.is_pressed() {
			self.player.move_horizontally(PLAYER_SPEED * time_delta_s);
		}

		self.player.advance_time(time_delta_s, &mut self.bullets);
		self.aliens.advance_time(time_delta_s);
		for bullet in &mut self.bullets {
			bullet.advance_time(time_delta_s);
		}
	}

	fn render(&mut self) {
		for bullet in &self.bullets {
			bullet.render(&mut self.window);
		}
		self.player.render(&mut self.window);
		self.aliens.render(&mut self.window);
	}
}
