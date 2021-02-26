pub mod config;
pub mod utilities;

use config::GraphicsConfig;
use piston::input::*;
use piston::window::Window;
use piston::Button;
use utilities::colors;

pub const UNIT_MOVE: f64 = 10.0;

pub struct App {
    pub window: GraphicsConfig,
    pub ship: Ship,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();
        let (x, y) = ((size.width / 2.0), (size.height / 2.0));

        let ship = Ship::new(x, y, 15.0);
        App { window, ship }
    }

    pub fn input(&mut self, button: Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::Up => self.ship.y = self.ship.y - UNIT_MOVE,
                Key::Down => self.ship.y = self.ship.y + UNIT_MOVE,
                Key::Left => self.ship.x = self.ship.x - UNIT_MOVE,
                Key::Right => self.ship.x = self.ship.x + UNIT_MOVE,
                _ => (),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.ship.size);
        let (x, y) = (self.ship.x, self.ship.y);
        self.window.gl.draw(args.viewport(), |c, gl| {
            clear(colors::BLACK, gl);

            let transform = c.transform.trans(x, y);

            rectangle(colors::RED, square, transform, gl);
        });
    }
}

pub struct Ship {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Ship {
    pub fn new(x: f64, y: f64, size: f64) -> Ship {
        Ship { x, y, size }
    }
}
