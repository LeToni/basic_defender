use super::GameObject;
use crate::geom;
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
