use super::GameObject;
use crate::utilities::colors;
use crate::utilities::geom::*;
use graphics::{polygon, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::window::Size;

const SHIP_SPEED: f64 = 2.0;
const SHIP_SIZE: f64 = 20.0;
const SHIP_DRIFT: f64 = 0.2;

pub struct Ship {
    pub pos: Position,
    pub dir: Direction,
    pub size: f64,
    pub drift_ttl: f64,
    move_offset: Position,
}

impl Ship {
    pub fn new(x: f64, y: f64, size: f64) -> Ship {
        Ship {
            pos: Position::new(x, y),
            dir: Direction::EAST,
            size: SHIP_SIZE,
            drift_ttl: 0.0,
            move_offset: Position::new(0.0, 0.0),
        }
    }

    pub fn start_move(&mut self, dir: Direction) {
        self.dir = dir;
        match dir {
            Direction::WEST => self.move_offset.x = -SHIP_SPEED,
            Direction::NORTH => self.move_offset.y = -SHIP_SPEED,
            Direction::EAST => self.move_offset.x = SHIP_SPEED,
            Direction::SOUTH => self.move_offset.y = SHIP_SPEED,
        }
    }

    pub fn stop_move(&mut self, dir: Direction) {
        self.drift_ttl = SHIP_DRIFT;
        match dir {
            Direction::WEST => self.move_offset.x = 0.0,
            Direction::NORTH => self.move_offset.y = 0.0,
            Direction::EAST => self.move_offset.x = 0.0,
            Direction::SOUTH => self.move_offset.y = 0.0,
        }
    }
}

impl GameObject for Ship {
    fn position(&self) -> &Position {
        &self.position()
    }

    fn radius(&self) -> f64 {
        self.size / 2.0
    }

    fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let shape = polygon::Polygon::new(colors::RED);

        let dir = match self.dir {
            Direction::WEST => 0.0,
            Direction::NORTH => 90.0,
            Direction::EAST => 180.0,
            Direction::SOUTH => 270.0,
        };

        let radius = self.radius();
        let transform = context
            .transform
            .trans(self.pos.x, self.pos.y)
            .rot_deg(dir)
            .trans(-radius, -radius);

        let points = [[0.0, radius], [self.size, self.size], [self.size, 0.0]];

        shape.draw(&points, &context.draw_state, transform, gl);
    }

    fn update(&mut self, dt: f64, size: Size) {
        let radius = self.radius();

        self.pos.x = self.pos.x + self.move_offset.x;
        self.pos.y = self.pos.y + self.move_offset.y;

        if self.drift_ttl > 0.0 {
            self.drift_ttl = self.drift_ttl - dt;
            let drift_speed = SHIP_SPEED / 2.0;
            match self.dir {
                Direction::NORTH => self.pos.y = self.pos.y - drift_speed,
                Direction::EAST => self.pos.x = self.pos.x + drift_speed,
                Direction::SOUTH => self.pos.y = self.pos.y + drift_speed,
                Direction::WEST => self.pos.x = self.pos.x - drift_speed,
            }
        }

        geom::restrict_to_bounds(
            &mut self.pos,
            [
                radius,
                radius,
                f64::from(size.width),
                f64::from(size.height),
            ],
        );
    }
}
