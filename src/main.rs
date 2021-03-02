use piston;
use piston::event_loop::*;
use piston::input::*;

use defender::config::GraphicsConfig;
use defender::App;

fn main() {
    let window = GraphicsConfig::new("Basic Defender", 960, 768);
    // Create a new game and run it.
    let mut app = App::new(window);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(key) = e.press_args() {
            app.input(key, true);
        }

        if let Some(key) = e.release_args() {
            app.input(key, false);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
