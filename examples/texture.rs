extern crate rendust;

use rendust::Color;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, Texture, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Texture", 1280, 720, true).unwrap();

    let vert = r#"
        #version 140

        in vec3 position;
        in vec4 color;
        in vec2 texcoords;

        out vec4 frag_color;
        out vec2 frag_texcoords;

        void main() {
            gl_Position = vec4(position, 1.0);

            frag_color = color;
            frag_texcoords = texcoords;
        }
    "#;

    let frag = r#"
        #version 140

        in vec4 frag_color;
        in vec2 frag_texcoords;

        out vec4 out_color;

        void main() {
            out_color = frag_color;
        }
    "#;

    let program = Program::new(vert, frag).unwrap();

    let texture = Texture::new(2, 2, &[
        255, 0, 0, 255,
        0, 255, 0, 255,
        0, 0, 255, 255,
        255, 255, 255, 255u8,
    ]);

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::new([-1.0, -1.0, 0.0], Color::new(1.0, 1.0, 1.0, 1.0)),
        Vertex::new([ 0.0,  1.0, 0.0], Color::new(1.0, 1.0, 1.0, 1.0)),
        Vertex::new([ 1.0, -1.0, 0.0], Color::new(1.0, 1.0, 1.0, 1.0)),
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
