extern crate rendust;

use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Triangle", 1280, 720, true).unwrap();

    let program = Program::basic().unwrap();

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::new(-1.0, -1.0, 0.0).color(1.0, 0.0, 0.0, 1.0),
        Vertex::new( 0.0,  1.0, 0.0).color(0.0, 1.0, 0.0, 1.0),
        Vertex::new( 1.0, -1.0, 0.0).color(0.0, 0.0, 1.0, 1.0),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color([0.0, 0.0, 0.0, 1.0]);
        rendust::clear();

        program.bind();
        triangle.render();

        window.swap_buffers();
    }
}
