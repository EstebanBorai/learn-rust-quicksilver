use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background, Color};
use quicksilver::lifecycle::Window;

use crate::bolt::Bolt;
use crate::MOVE_LIMIT;

const PLAYER_BLOCK_SIZE: f32 = 5.;
const SPEED: f32 = 5.;

pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub bolts: Vec<Bolt>,
}

pub enum Direction {
    Right,
    Left,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos_x: 150.,
            pos_y: 260.,
            bolts: Vec::new(),
        }
    }

    pub fn moveto(&mut self, dir: Direction) {
        match dir {
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

    pub fn get_position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    pub fn shoot(&mut self) {
        let (x, y) = self.get_position();
        self.bolts.push(Bolt::from_player_pos(x, y));
    }

    pub fn draw(&mut self, window: &mut Window) {
        println!("Active Bolts: {}", self.bolts.len());

        window.draw_ex(
            &Rectangle::new(
                (self.pos_x, self.pos_y),
                (PLAYER_BLOCK_SIZE, PLAYER_BLOCK_SIZE),
            ),
            Background::Col(Color::GREEN),
            Transform::translate(Vector::new(self.pos_x, self.pos_y)),
            0,
        );

        // Bolts to be deleted at the end of this animation
        // loop frame
        let mut delete_next_bolts = Vec::new();

        self.bolts.iter_mut().enumerate().for_each(|(idx, b)| {
            if b.draw(window) {
                delete_next_bolts.push(idx);
            }
        });

        for idx in delete_next_bolts {
            self.bolts.remove(idx);
        }
    }
}
