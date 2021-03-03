pub mod bullet;
pub mod enemy;
pub mod ship;

use crate::geom::Position;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::window::Size;

pub trait GameObject {
    fn position(&self) -> &Position;
    fn radius(&self) -> f64;

    fn collides(&self, other: &dyn GameObject) -> bool {
        let dx = self.position().x - other.position().x;
        let dy = self.position().y - other.position().y;
        let sum = dx.powf(2.0) + dy.powf(2.0);

        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();

        return r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0);
    }

    fn render(&self, context: &Context, gl: &mut GlGraphics);
    fn update(&mut self, _: f64, _: &Size) {}
}
