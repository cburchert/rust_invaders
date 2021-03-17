use crate::settings::{
	INITIAL_GUN_COOLDOWN_S, PLAYER_H, PLAYER_PADDING, PLAYER_W, SCREEN_H, SCREEN_W,
};
use sfml::graphics::{ConvexShape, Drawable, RenderStates, RenderWindow};
use sfml::system::Vector2f;

pub struct Player {
	position: Vector2f,
	size: Vector2f,
	gun_cooldown: f32,
}

impl Player {
	pub fn new() -> Player {
		Player {
			position: Vector2f::new(
				(SCREEN_W as f32 + PLAYER_W) / 2.,
				SCREEN_H as f32 - PLAYER_PADDING,
			),
			size: Vector2f::new(PLAYER_W, PLAYER_H),
			gun_cooldown: INITIAL_GUN_COOLDOWN_S,
		}
	}

	pub fn render(&self, window: &mut RenderWindow) {
		let mut triangle = ConvexShape::new(5);
		triangle.set_point(0, (self.position.x + self.size.x / 2., self.position.y));
		triangle.set_point(
			1,
			(
				self.position.x + self.size.x,
				self.position.y + self.size.y / 2.,
			),
		);
		triangle.set_point(
			2,
			(self.position.x + self.size.x, self.position.y + self.size.y),
		);
		triangle.set_point(3, (self.position.x, self.position.y + self.size.y));
		triangle.set_point(4, (self.position.x, self.position.y + self.size.y / 2.));

		triangle.draw(window, RenderStates::default());
	}
}
