extern crate rendust;

use rendust::window::Window;

fn main() {
    let mut window = Window::new("Rendust example - Basic", 1280, 720).unwrap();

    while !window.should_exit {
        window.handle_events(|_| ());
        window.swap_buffers();
    }
}
