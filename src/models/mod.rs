pub mod ship;

use crate::utilities::geom::Position;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::window::Size;

pub trait GameObject {
    fn position(&self) -> &Position;
    fn radius(&self) -> f64;

    fn render(&self, context: &Context, gl: &mut GlGraphics);
    fn update(&mut self, _: f64, _: Size) {}
}
