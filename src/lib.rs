pub mod colors;
pub mod config;
pub mod geom;
pub mod models;
pub mod utils;

use crate::models::bullet::Bullet;
use crate::models::enemy::Enemy;
use crate::models::ship::Ship;
use crate::models::GameObject;
use crate::utils::{draw_text, draw_text_center};
use config::GraphicsConfig;
use opengl_graphics::{GlyphCache, TextureSettings};
use piston::input::*;
use piston::window::Window;
use piston::Button;

pub const UNIT_MOVE: f64 = 10.0;
pub const FIRE_COOLDOWN: f64 = 0.1;

pub struct GameState {
    fire_bullets: bool,
    fire_cooldown: f64,
}

enum GameStatus {
    Normal,
    Destroyed,
    Win,
}

pub struct App<'a> {
    pub window: GraphicsConfig,
    glyph_cache: GlyphCache<'a>,
    pub ship: Ship,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    state: GameState,
    status: GameStatus,
    score: u16,
}

impl<'a> App<'a> {
    pub fn new(window: GraphicsConfig) -> App<'a> {
        let size = window.settings.size();
        let (x, y) = ((size.width / 2.0), (size.height / 2.0));

        let ship = Ship::new(x, y);
        let state = GameState {
            fire_bullets: false,
            fire_cooldown: 0.0,
        };

        let glyph_cache = GlyphCache::new(
            "./assets/fonts/PxPlus_IBM_VGA8.ttf",
            (),
            TextureSettings::new(),
        )
        .expect("Unable to load font");

        App {
            window,
            glyph_cache,
            ship,
            enemies: Vec::new(),
            bullets: Vec::new(),
            state,
            status: GameStatus::Normal,
            score: 0,
        }
    }

    fn reset(&mut self) {
        self.status = GameStatus::Normal;
        self.score = 0;
        self.enemies.clear();
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
                    Key::Return => match self.status {
                        GameStatus::Destroyed => self.reset(),
                        GameStatus::Win => self.reset(),
                        _ => (),
                    },
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
        let score = &self.score;
        let status = &self.status;
        let glyph_cache = &mut self.glyph_cache;
        let size = self.window.size;

        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(colors::BLACK, gl);
            match status {
                GameStatus::Destroyed => {
                    draw_text_center(
                        "YOU DIED!",
                        32,
                        [f64::from(size.width), f64::from(size.height)],
                        glyph_cache,
                        &c,
                        gl,
                    );
                    return;
                }
                GameStatus::Win => {
                    draw_text_center(
                        "YOU WIN!",
                        32,
                        [f64::from(size.width), f64::from(size.height)],
                        glyph_cache,
                        &c,
                        gl,
                    );
                    return;
                }
                _ => (),
            }

            // Render the current score
            let score_str = format!("Score: {}", score);
            draw_text(score_str.as_str(), [0.0, 16.0], 16, glyph_cache, &c, gl);

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

        match self.status {
            GameStatus::Destroyed => return,
            GameStatus::Win => return,
            _ => (),
        }

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

            for enemy in &mut self.enemies {
                if bullet.collides(enemy) {
                    bullet.ttl = 0.0;
                    enemy.health = 0;
                    self.score = self.score + 1;
                }
            }
        }
        self.bullets.retain(|bullet| bullet.ttl > 0.0);
        self.enemies.retain(|enemy| enemy.health > 0);
        self.ship.update(args.dt, &size);

        if self.enemies.is_empty() {
            for _ in 0..10 {
                self.enemies.push(Enemy::new_rand(size.width, size.height));
            }
        }

        for enemy in &mut self.enemies {
            enemy.update(args.dt, &size);
            if enemy.collides(&self.ship) {
                self.status = GameStatus::Destroyed;
            }
        }

        if self.score == 10 {
            self.status = GameStatus::Win;
        }
    }
}
