use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background, Color};
use quicksilver::lifecycle::Window;

use crate::MOVE_LIMIT;

const BLOCK_SIZE: f32 = 5.;
const SPEED: f32 = 5.;

pub struct Bolt {
    pos_x: f32,
    pos_y: f32,
}

impl Bolt {
    pub fn from_player_pos(x: f32, y: f32) -> Self {
        Self {
            pos_x: x,
            pos_y: y - 10.,
        }
    }

    pub fn update_travel(&mut self) -> (f32, f32) {
        let (x, y) = (self.pos_x, self.pos_y);

        self.pos_y -= SPEED;

        (x, y)
    }

    /// Returns `true` if the `Bolt` have to be removed
    pub fn draw(&mut self, window: &mut Window) -> bool {
        let (x, y) = self.update_travel();

        if y < 0. {
            // When `y` is zero, means that this `Bolt` has
            // reached the end of the screen and should be
            // removed
            return true;
        }

        window.draw_ex(
            &Rectangle::new((x, y), (BLOCK_SIZE, BLOCK_SIZE + 10.)),
            Background::Col(Color::WHITE),
            Transform::translate(Vector::new(x, y)),
            0,
        );

        false
    }
}
