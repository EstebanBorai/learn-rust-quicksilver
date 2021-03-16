use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background, Color};
use quicksilver::input::Key;
use quicksilver::lifecycle::{run, Settings, State, Window};
use quicksilver::Result;

use crate::bolt::Bolt;
use crate::enemy::Enemy;
use crate::player::Player;

mod bolt;
mod enemy;
mod player;

pub const MOVE_LIMIT: f32 = 300.;

const WINDOW_SQUARE_LENGTH: f32 = 600.;
const UPDATE_RATE: f64 = 40.;

fn main() {
    run::<Game>(
        "Enemies Respond Attack",
        Vector::new(WINDOW_SQUARE_LENGTH, WINDOW_SQUARE_LENGTH),
        Settings {
            draw_rate: UPDATE_RATE,
            update_rate: UPDATE_RATE,
            ..Settings::default()
        },
    );
}

struct Game {
    enemy: Enemy,
    player: Player,
    player_bolts: Vec<Bolt>,
}

impl Game {
    pub fn update_player_attack(&mut self) {
        if self
            .player_bolts
            .iter()
            .any(|b| b.pos_x == self.enemy.pos_x && b.pos_y == self.enemy.pos_y)
        {
            self.enemy.kill();
        }
    }

    pub fn draw_player_bolts(&mut self, window: &mut Window) {
        let player_pos = self.player.get_position();
        println!("Active Bolts: {}", self.player_bolts.len());

        window.draw_ex(
            &Rectangle::new(
                (player_pos.0, player_pos.1),
                (player::PLAYER_BLOCK_SIZE, player::PLAYER_BLOCK_SIZE),
            ),
            Background::Col(Color::GREEN),
            Transform::translate(Vector::new(player_pos.0, player_pos.1)),
            0,
        );

        // Bolts to be deleted at the end of this animation
        // loop frame
        let mut delete_next_bolts = Vec::new();

        self.player_bolts
            .iter_mut()
            .enumerate()
            .for_each(|(idx, b)| {
                if b.draw(window) {
                    delete_next_bolts.push(idx);
                }
            });

        for idx in delete_next_bolts {
            self.player_bolts.remove(idx);
        }
    }
}

impl State for Game {
    fn new() -> Result<Self> {
        Ok(Self {
            enemy: enemy::Enemy::new(),
            player: player::Player::new(),
            player_bolts: Vec::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() {
            self.player.moveto(player::Direction::Right);
        }

        if window.keyboard()[Key::Left].is_down() {
            self.player.moveto(player::Direction::Left);
        }

        if window.keyboard()[Key::Space].is_down() {
            let (x, y) = self.player.get_position();
            self.player_bolts.push(Bolt::from_player_pos(x, y));
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.player.draw(window);
        self.enemy.draw(window);
        self.draw_player_bolts(window);
        self.update_player_attack();

        Ok(())
    }
}
