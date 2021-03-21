use crate::bullet::Bullet;
use crate::settings::{
	INITIAL_GUN_COOLDOWN_S, PLAYER_H, PLAYER_PADDING, PLAYER_W, SCREEN_H, SCREEN_W,
};

use sfml::graphics::{
	Color, ConvexShape, Drawable, RenderStates, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2f;
use sfml::window::Key;

pub struct Player {
	position: Vector2f,
	size: Vector2f,
	gun_cooldown: f32,
	gun_cooldown_left: f32,
}

impl Player {
	pub fn new() -> Player {
		Player {
			position: Vector2f::new(
				(SCREEN_W as f32 - PLAYER_W) / 2.,
				SCREEN_H as f32 - PLAYER_PADDING - PLAYER_H,
			),
			size: Vector2f::new(PLAYER_W, PLAYER_H),
			gun_cooldown: INITIAL_GUN_COOLDOWN_S,
			gun_cooldown_left: 0.,
		}
	}

	pub fn advance_time(&mut self, time_delta_s: f32, bullets: &mut Vec<Bullet>) {
		self.gun_cooldown_left -= time_delta_s;

		if self.gun_cooldown_left <= 0. && Key::Space.is_pressed() {
			self.gun_cooldown_left = self.gun_cooldown;

			let bullet_pos = self.position + Vector2f::new(self.size.x * 0.5, 0.);
			bullets.push(Bullet::new(bullet_pos));
		}
	}

	pub fn render(&self, window: &mut RenderWindow) {
		Player::draw_player_shape(window, self.position, self.size)
	}

	fn draw_player_shape(window: &mut RenderWindow, position: Vector2f, size: Vector2f) {
		let mut center_body = ConvexShape::new(5);
		center_body.set_point(0, (0.5, 0.));
		center_body.set_point(1, (1., 0.5));
		center_body.set_point(2, (1., 1.));
		center_body.set_point(3, (0., 1.));
		center_body.set_point(4, (0., 0.5));
		center_body.set_scale(size);
		center_body.set_position(position);
		center_body.set_fill_color(Color::RED);
		center_body.draw(window, RenderStates::default());
	}

	pub fn move_horizontally(&mut self, delta: f32) {
		self.position.x = (self.position.x + delta).clamp(0., SCREEN_W as f32 - PLAYER_W);
	}
}
