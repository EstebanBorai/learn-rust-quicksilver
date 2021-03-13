use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use quicksilver::input::Key;
use quicksilver::lifecycle::{run, Settings, State, Window};
use quicksilver::Result;

mod bolt;
mod enemy;
mod player;

pub const MOVE_LIMIT: f32 = 300.;

const WINDOW_SQUARE_LENGTH: f32 = 600.;
const UPDATE_RATE: f64 = 40.;

fn main() {
    run::<Game>(
        "Actions",
        Vector::new(WINDOW_SQUARE_LENGTH, WINDOW_SQUARE_LENGTH),
        Settings {
            draw_rate: UPDATE_RATE,
            update_rate: UPDATE_RATE,
            ..Settings::default()
        },
    );
}

struct Game {
    enemy: enemy::Enemy,
    player: player::Player,
}

fn is_collision(enemy: &enemy::Enemy, player: &player::Player) -> bool {
    enemy.pos_x == player.pos_x && enemy.pos_y == player.pos_y
}

impl State for Game {
    fn new() -> Result<Self> {
        Ok(Self {
            enemy: enemy::Enemy::new(),
            player: player::Player::new(),
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
            self.player.shoot();
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.player.draw(window);
        self.enemy.draw(window);

        if is_collision(&self.enemy, &self.player) {
            println!("Collision Found! Resetting Game!");
            self.player = player::Player::new();
            self.enemy = enemy::Enemy::new();
        }

        Ok(())
    }
}
