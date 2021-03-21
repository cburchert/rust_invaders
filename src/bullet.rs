use crate::settings::{BULLET_HEIGHT, BULLET_SPEED, BULLET_WIDTH};
use ::sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use ::sfml::system::Vector2f;

pub struct Bullet {
    position: Vector2f,
    size: Vector2f,
    velocity: Vector2f,
}

impl Bullet {
    pub fn new(position: Vector2f) -> Bullet {
        Bullet {
            position,
            size: Vector2f::new(BULLET_WIDTH, BULLET_HEIGHT),
            velocity: Vector2f::new(0., -BULLET_SPEED),
        }
    }

    pub fn advance_time(&mut self, time_delta_s: f32) {
        self.position += self.velocity * time_delta_s;
    }

    pub fn render(&self, window: &mut RenderWindow) {
        let mut bullet_rect = RectangleShape::with_size(self.size);
        bullet_rect.set_position(self.position);
        bullet_rect.set_fill_color(Color::RED);
        window.draw(&bullet_rect);
    }
}
