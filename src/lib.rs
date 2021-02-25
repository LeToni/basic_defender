use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;

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

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        self.window.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

pub struct GraphicsConfig {
    pub gl: GlGraphics,
    pub settings: GlutinWindow,
}

impl GraphicsConfig {
    pub fn new(title: &str, width: u32, height: u32) -> GraphicsConfig {
        let opengl = OpenGL::V4_5;
        // Create an Glutin window.
        let settings = WindowSettings::new(title, [width, height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        return GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings,
        };
    }
}
