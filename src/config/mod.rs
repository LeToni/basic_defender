use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::Size;
use piston::window::WindowSettings;

pub struct GraphicsConfig {
    pub gl: GlGraphics,
    pub settings: GlutinWindow,
    pub size: Size,
}

impl GraphicsConfig {
    pub fn new(title: &str, width: u32, height: u32) -> GraphicsConfig {
        let opengl = OpenGL::V4_5;
        // Create an Glutin window.
        let size = Size {
            width: width as f64,
            height: height as f64,
        };
        let settings = WindowSettings::new(title, [width, height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        return GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings,
            size,
        };
    }
}
