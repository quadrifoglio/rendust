extern crate rendust;

use rendust::Color;
use rendust::window::Window;

fn main() {
    let mut window = Window::new("Rendust example - Basic", 1280, 720, true).unwrap();

    rendust::set_clear_color(Color::new(0.0, 0.0, 1.0, 1.0));

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::clear();
        window.swap_buffers();
    }
}
