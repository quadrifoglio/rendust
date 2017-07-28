extern crate rendust;
extern crate imagefmt;

use rendust::Color;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, Texture, PrimitiveType, Mesh};

fn main() {
    let image = imagefmt::read("examples/image.png", imagefmt::ColFmt::RGBA).unwrap();

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

        uniform sampler2D tex;

        in vec4 frag_color;
        in vec2 frag_texcoords;

        out vec4 out_color;

        void main() {
            out_color = texture2D(tex, frag_texcoords);
        }
    "#;

    let program = Program::new(vert, frag).unwrap();

    let texture = Texture::new(image.w as u32, image.h as u32, image.buf.as_ref());

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::textured([-1.0, -1.0, 0.0], [0.0, 0.0]),
        Vertex::textured([ 0.0,  1.0, 0.0], [0.5, 1.0]),
        Vertex::textured([ 1.0, -1.0, 0.0], [1.0, 0.0]),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color(Color::new(0.0, 0.0, 0.0, 1.0));
        rendust::clear();

        program.bind();
        texture.bind();

        triangle.render();

        window.swap_buffers();
    }
}
