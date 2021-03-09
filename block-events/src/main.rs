use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background, Color};
use quicksilver::input::Key;
use quicksilver::lifecycle::{run, Settings, State, Window};
use quicksilver::Result;

const BLOCK_SIZE: f32 = 5.;
const MOVE_LIMIT: f32 = 300.;
const SPEED: f32 = 5.;
const WINDOW_SQUARE_LENGTH: f32 = 600.;

fn main() {
    run::<GameState>(
        "Block Events",
        Vector::new(WINDOW_SQUARE_LENGTH, WINDOW_SQUARE_LENGTH),
        Settings {
            draw_rate: 40.,
            update_rate: 40.,
            ..Settings::default()
        },
    );
}

struct GameState {
    x: f32,
    y: f32,
}

impl State for GameState {
    fn new() -> Result<Self> {
        Ok(Self {
            x: MOVE_LIMIT / 2.,
            y: MOVE_LIMIT / 2.,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() && self.x + SPEED < MOVE_LIMIT {
            self.x += SPEED;
        }

        if window.keyboard()[Key::Left].is_down() && self.x - SPEED > 0. {
            self.x -= SPEED;
        }

        if window.keyboard()[Key::Up].is_down() && (self.y - SPEED) > 0. {
            self.y -= SPEED;
        }

        if window.keyboard()[Key::Down].is_down() && (self.y + SPEED) < MOVE_LIMIT {
            self.y += SPEED;
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.draw_ex(
            &Rectangle::new((self.x, self.y), (BLOCK_SIZE, BLOCK_SIZE)),
            Background::Col(Color::GREEN),
            Transform::translate(Vector::new(self.x, self.y)),
            0,
        );

        Ok(())
    }
}
