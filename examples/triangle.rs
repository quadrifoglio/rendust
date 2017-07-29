extern crate rendust;

use rendust::Context;
use rendust::window::Window;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Triangle", 1280, 720, true).unwrap();
    
    // A context must to created to initialize the shader program
    let _ = Context::new().unwrap();

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::new(-1.0, -1.0, 0.0).color(1.0, 0.0, 0.0, 1.0),
        Vertex::new( 0.0,  1.0, 0.0).color(0.0, 1.0, 0.0, 1.0),
        Vertex::new( 1.0, -1.0, 0.0).color(0.0, 0.0, 1.0, 1.0),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color(0.0, 0.0, 0.0, 1.0);
        rendust::clear();

        triangle.render();

        window.swap_buffers();
    }
}
