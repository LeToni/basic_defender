use crate::utilities::colors;
use graphics::*;
use opengl_graphics::GlGraphics;

pub struct Ship {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}
impl Ship {
    pub fn new(x: f64, y: f64, size: f64) -> Ship {
        Ship { x, y, size }
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let transform = context.transform.trans(self.x, self.y);
        let square = rectangle::square(0.0, 0.0, self.size);
        rectangle(colors::RED, square, transform, gl);
    }
}
