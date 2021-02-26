pub mod config;
pub mod utilities;

use config::GraphicsConfig;
use piston::input::*;
use piston::window::Window;
use piston::Button;
use utilities::colors;

pub struct App {
    pub window: GraphicsConfig,
    pub x: f64,
    pub y: f64,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();
        let (x, y) = ((size.width / 2.0), (size.height / 2.0));

        App { window, x, y }
    }

    pub fn input(&mut self, button: Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::Up => self.y = self.y - 10.0,
                Key::Down => self.y = self.y + 10.0,
                Key::Left => self.x = self.x - 10.0,
                Key::Right => self.x = self.x + 10.0,
                _ => (),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 15.0);
        let (x, y) = (self.x, self.y);
        // let rotation = self.rotation;
        self.window.gl.draw(args.viewport(), |c, gl| {
            clear(colors::BLACK, gl);

            let transform = c.transform.trans(x, y);
            // .rot_rad(rotation)
            // .trans(-25.0, -25.0);

            rectangle(colors::RED, square, transform, gl);
        });
    }

    // pub fn update(&mut self, args: &UpdateArgs) {
    //     // self.rotation += 2.0 * args.dt;
    // }
}
