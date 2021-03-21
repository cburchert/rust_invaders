use crate::matrix::Matrix;
use crate::settings::{
    ALIEN_GROUP_X_PADDING, ALIEN_GROUP_Y_PADDING, ALIEN_PADDING, ALIEN_SPEED, ALIEN_Y_STEP,
    SCREEN_W,
};
use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;

pub struct AlienGroup {
    num_x: usize,
    num_y: usize,
    is_alive: Matrix<bool>,
    position: Vector2f,
    size: Vector2f,
    speed: f32,
}

impl AlienGroup {
    pub fn new(num_x: usize, num_y: usize) -> AlienGroup {
        // Center the group.
        let width = SCREEN_W as f32 - ALIEN_GROUP_X_PADDING * 2.;
        // Adjust height, so the aliens are squares.
        let height = width / num_x as f32 * num_y as f32;
        AlienGroup {
            is_alive: Matrix::new(num_y, num_x, true),
            num_x,
            num_y,
            position: Vector2f::new(ALIEN_GROUP_X_PADDING, ALIEN_GROUP_Y_PADDING),
            size: Vector2f::new(width, height),
            speed: ALIEN_SPEED,
        }
    }

    pub fn render(&mut self, window: &mut RenderWindow) {
        for y_index in 0..self.num_y {
            for x_index in 0..self.num_x {
                if *self.is_alive.at(y_index, x_index) {
                    let mut alien_pos = self.position;
                    alien_pos.x += x_index as f32 * self.size.x / self.num_x as f32;
                    alien_pos.y += y_index as f32 * self.size.y / self.num_y as f32;

                    let alien_size = Vector2f::new(
                        self.size.x / self.num_x as f32,
                        self.size.y / self.num_y as f32,
                    );

                    AlienGroup::render_one(alien_pos, alien_size, window);
                }
            }
        }
    }

    pub fn advance_time(&mut self, time_delta_s: f32) {
        self.position.x += self.speed * time_delta_s;
        if self.position.x + self.size.x > SCREEN_W as f32 {
            self.speed = -ALIEN_SPEED;
            self.position.y += ALIEN_Y_STEP;
        } else if self.position.x < 0. {
            self.speed = ALIEN_SPEED;
            self.position.y += ALIEN_Y_STEP;
        }
    }

    fn render_one(position: Vector2f, size: Vector2f, window: &mut RenderWindow) {
        let position_padded = position + size * ALIEN_PADDING;
        // We need to subtract the padding twice to get it on both sides.
        let size_unpadded = size * (1. - ALIEN_PADDING * 2.);

        let mut alien_rect = RectangleShape::with_size(size_unpadded);
        alien_rect.set_fill_color(Color::BLUE);
        alien_rect.set_position(position_padded);
        window.draw(&alien_rect);
    }
}
