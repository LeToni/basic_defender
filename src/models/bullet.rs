use super::GameObject;
use crate::colors;
use crate::geom;
use graphics::{ellipse, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::window::Size;

const BULLET_SPEED: f64 = 2.0;
const BULLET_SIZE: f64 = 5.0;
const BULLET_LIFETIME: f64 = 2.0;

pub struct Bullet {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub size: f64,
    pub ttl: f64,
}

impl Bullet {
    pub fn new {x: f64, y: f64, dir: geom::Direction} -> Bullet{
        Bullet{pos: geom::Position{x,y}, dir,size: BULLET_SIZE,ttl: BULLET_LIFETIME}
    }

    pub fn radius(&self) -> f64{
        self.size/2.0
    }
}