pub mod colors;
pub mod config;
pub mod geom;
pub mod models;

use crate::models::GameObject;
// use crate::models::bullet::Bullet;
// use crate::models::enemy::Enemy;
use crate::models::ship::Ship;
use config::GraphicsConfig;
use piston::input::*;
use piston::window::Window;
use piston::Button;

pub const UNIT_MOVE: f64 = 10.0;

pub struct App {
    pub window: GraphicsConfig,
    pub ship: Ship,
    // enemies: Vec<Enemy>,
    // bullets: Vec<Bullet>,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();
        let (x, y) = ((size.width / 2.0), (size.height / 2.0));

        let ship = Ship::new(x, y);
        App { window, ship }
    }

    pub fn input(&mut self, button: Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Up => self.ship.start_move(geom::Direction::NORTH),
                    Key::Down => self.ship.start_move(geom::Direction::SOUTH),
                    Key::Left => self.ship.start_move(geom::Direction::WEST),
                    Key::Right => self.ship.start_move(geom::Direction::EAST),
                    _ => (),
                }
            }
        } else if let Button::Keyboard(key) = button {
            match key {
                Key::Up => self.ship.stop_move(geom::Direction::NORTH),
                Key::Down => self.ship.stop_move(geom::Direction::SOUTH),
                Key::Left => self.ship.stop_move(geom::Direction::WEST),
                Key::Right => self.ship.stop_move(geom::Direction::EAST),
                _ => (),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let ship = &self.ship;
        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(colors::BLACK, gl);

            ship.render(&c, gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        let size = self.window.size;
        self.ship.update(args.dt, size)
    }
}
