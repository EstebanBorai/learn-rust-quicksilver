use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background, Color};
use quicksilver::lifecycle::Window;

use crate::MOVE_LIMIT;

const PLAYER_BLOCK_SIZE: f32 = 5.;
const SPEED: f32 = 5.;

pub struct Enemy {
    pub is_active: bool,
    pub pos_x: f32,
    pub pos_y: f32,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            is_active: true,
            pos_x: 100.,
            pos_y: 100.,
        }
    }

    pub fn kill(&mut self) {
        self.is_active = false;
    }

    pub fn moveto(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                if self.pos_y - SPEED > 0. {
                    self.pos_y -= SPEED;
                }
            }
            Direction::Down => {
                if self.pos_y + SPEED < MOVE_LIMIT {
                    self.pos_y += SPEED
                }
            }
            Direction::Right => {
                if self.pos_x + SPEED < MOVE_LIMIT {
                    self.pos_x += SPEED
                }
            }
            Direction::Left => {
                if self.pos_x - SPEED > 0. {
                    self.pos_x -= SPEED
                }
            }
        }
    }

    pub fn draw(&self, window: &mut Window) {
        if self.is_active {
            window.draw_ex(
                &Rectangle::new(
                    (self.pos_x, self.pos_y),
                    (PLAYER_BLOCK_SIZE, PLAYER_BLOCK_SIZE),
                ),
                Background::Col(Color::RED),
                Transform::translate(Vector::new(self.pos_x, self.pos_y)),
                0,
            );
        }
    }
}
