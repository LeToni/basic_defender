use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

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
