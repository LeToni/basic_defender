use super::GameObject;
use crate::colors;
use crate::geom;
use graphics::{rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::window::Size;
use rand;
use rand::Rng;

const MOVE_RADIUS: f64 = 5.0;
const MOVE_TTL: f64 = 0.1; // 100 millisecond
const ENEMY_RADIUS: f64 = 10.0;

pub struct Enemy {
    pub pos: geom::Position,
    pub size: f64,
    pub health: i8,
    move_ttl: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy {
            pos: geom::Position { x, y },
            size: ENEMY_RADIUS * 2.0,
            health: 1,
            move_ttl: MOVE_TTL,
        }
    }
    pub fn new_rand(max_x: f64, max_y: f64) -> Enemy {
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0..max_x);
        let randy = rng.gen_range(0.0..max_y);
        Enemy::new(randx, randy)
    }
}

impl GameObject for Enemy {
    fn position(&self) -> &geom::Position {
        &self.pos
    }

    fn radius(&self) -> f64 {
        self.size / 2.0
    }

    fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = context
            .transform
            .trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        rectangle(colors::GREEN, square, transform, gl);
    }

    fn update(&mut self, dt: f64, size: Size) {
        self.move_ttl = self.move_ttl - dt;
        if self.move_ttl <= 0.0 {
            let radius = self.radius();
            let mut rng = rand::thread_rng();

            self.pos.x = self.pos.x + rng.gen_range(0.0..MOVE_RADIUS * 2.0) - MOVE_RADIUS;
            self.pos.y = self.pos.y + rng.gen_range(0.0..MOVE_RADIUS * 2.0) - MOVE_RADIUS;

            geom::restrict_to_bounds(&mut self.pos, [radius, radius, size.width, size.height]);

            self.move_ttl = MOVE_TTL;
        }
    }
}
