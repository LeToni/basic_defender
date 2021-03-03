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
    pub fn new(x: f64, y: f64, dir: geom::Direction) -> Bullet {
        Bullet {
            pos: geom::Position { x, y },
            dir,
            size: BULLET_SIZE,
            ttl: BULLET_LIFETIME,
        }
    }

    pub fn radius(&self) -> f64 {
        self.size / 2.0
    }
}

impl GameObject for Bullet {
    fn position(&self) -> &geom::Position {
        &self.pos
    }
    fn radius(&self) -> f64 {
        BULLET_SIZE
    }

    fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let transform = context.transform.trans(self.pos.x, self.pos.y);
        let radius = self.radius();
        ellipse(colors::WHITE, [0.0, 0.0, radius, radius], transform, gl);
    }

    fn update(&mut self, dt: f64, _: &Size) {
        self.ttl = self.ttl - dt;
        // Move the bullet in the direction the player was facing.
        match self.dir {
            geom::Direction::EAST => self.pos.x = self.pos.x + BULLET_SPEED,
            geom::Direction::NORTH => self.pos.y = self.pos.y - BULLET_SPEED,
            geom::Direction::WEST => self.pos.x = self.pos.x - BULLET_SPEED,
            geom::Direction::SOUTH => self.pos.y = self.pos.y + BULLET_SPEED,
        }
    }
}
