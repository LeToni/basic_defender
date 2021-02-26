use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::PressEvent;

use defender::config::GraphicsConfig;
use defender::App;

fn main() {
    let window = GraphicsConfig::new("Basic Defender", 960, 768);
    // Create a new game and run it.
    let mut app = App::new(window);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(key) = e.press_args() {
            app.input(key);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
