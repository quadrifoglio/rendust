extern crate rendust;

use rendust::Color;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Triangle", 1280, 720, true).unwrap();

    let vert = r#"
        #version 140

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let frag = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let program = Program::new(vert, frag).unwrap();

    let triangle = Mesh::new(&[
        Vertex{ position: [-1.0, -1.0, 0.0] },
        Vertex{ position: [ 0.0,  1.0, 0.0] },
        Vertex{ position: [ 1.0, -1.0, 0.0] },
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
