extern crate rendust;

use rendust::Color;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Triangle", 1280, 720, true).unwrap();

    let vert = r#"
        #version 140

        in vec3 position;
        in vec4 color;

        out vec4 frag_color;

        void main() {
            gl_Position = vec4(position, 1.0);
            frag_color = color;
        }
    "#;

    let frag = r#"
        #version 140

        in vec4 frag_color;
        out vec4 out_color;

        void main() {
            out_color = frag_color;
        }
    "#;

    let program = Program::new(vert, frag).unwrap();

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::new([-1.0, -1.0, 0.0], Color::new(1.0, 0.0, 0.0, 1.0)),
        Vertex::new([ 0.0,  1.0, 0.0], Color::new(0.0, 1.0, 0.0, 1.0)),
        Vertex::new([ 1.0, -1.0, 0.0], Color::new(0.0, 0.0, 1.0, 1.0)),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        rendust::clear();

        program.bind();
        triangle.render();

        window.swap_buffers();
    }
}
