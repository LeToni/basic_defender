pub mod config;
pub mod utilities;

use config::GraphicsConfig;
use piston::input::{RenderArgs, UpdateArgs};
use utilities::colors;

pub struct App {
    pub window: GraphicsConfig,
    rotation: f64,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        App {
            window,
            rotation: 0.0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        self.window.gl.draw(args.viewport(), |c, gl| {
            clear(colors::BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            rectangle(colors::RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}
