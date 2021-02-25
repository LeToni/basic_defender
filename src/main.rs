use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};

use defender::config::GraphicsConfig;
use defender::App;

fn main() {
    let window = GraphicsConfig::new("Basic Defender", 200, 200);
    // Create a new game and run it.
    let mut app = App::new(window);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
