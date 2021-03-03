pub mod colors;
pub mod config;
pub mod geom;
pub mod models;

use crate::models::bullet::Bullet;
use crate::models::enemy::Enemy;
use crate::models::ship::Ship;
use crate::models::GameObject;
use config::GraphicsConfig;
use piston::input::*;
use piston::window::Window;
use piston::Button;

pub const UNIT_MOVE: f64 = 10.0;
pub const FIRE_COOLDOWN: f64 = 0.1;

pub struct GameState {
    fire_bullets: bool,
    fire_cooldown: f64,
}

pub struct App {
    pub window: GraphicsConfig,
    pub ship: Ship,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    state: GameState,
}

impl App {
    pub fn new(window: GraphicsConfig) -> App {
        let size = window.settings.size();
        let (x, y) = ((size.width / 2.0), (size.height / 2.0));

        let ship = Ship::new(x, y);
        let state = GameState {
            fire_bullets: false,
            fire_cooldown: 0.0,
        };
        App {
            window,
            ship,
            enemies: Vec::new(),
            bullets: Vec::new(),
            state,
        }
    }

    pub fn input(&mut self, button: Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Up => self.ship.start_move(geom::Direction::NORTH),
                    Key::Down => self.ship.start_move(geom::Direction::SOUTH),
                    Key::Left => self.ship.start_move(geom::Direction::WEST),
                    Key::Right => self.ship.start_move(geom::Direction::EAST),
                    Key::Space => {
                        if self.state.fire_cooldown <= 0.0 {
                            self.state.fire_cooldown = FIRE_COOLDOWN;
                            self.state.fire_bullets = true;
                        }
                    }
                    _ => (),
                }
            }
        } else if let Button::Keyboard(key) = button {
            match key {
                Key::Up => self.ship.stop_move(geom::Direction::NORTH),
                Key::Down => self.ship.stop_move(geom::Direction::SOUTH),
                Key::Left => self.ship.stop_move(geom::Direction::WEST),
                Key::Right => self.ship.stop_move(geom::Direction::EAST),
                _ => (),
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let ship = &self.ship;
        let enemies = &self.enemies;
        let bullets = &self.bullets;

        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(colors::BLACK, gl);

            for bullet in bullets.iter() {
                bullet.render(&c, gl);
            }

            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }

            ship.render(&c, gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        let size = self.window.size;

        if self.state.fire_cooldown > 0.0 {
            self.state.fire_cooldown = self.state.fire_cooldown - args.dt;
        }

        if self.state.fire_bullets {
            self.state.fire_bullets = false;
            self.bullets
                .push(Bullet::new(self.ship.pos.x, self.ship.pos.y, self.ship.dir));
        }

        for bullet in &mut self.bullets {
            bullet.update(args.dt, &size);
        }

        self.ship.update(args.dt, &size);

        if self.enemies.is_empty() {
            for _ in 0..10 {
                self.enemies.push(Enemy::new_rand(size.width, size.height));
            }
        }

        self.bullets.retain(|bullet| bullet.ttl > 0.0);
    }
}
