extern crate rendust;
extern crate imagefmt;

use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, Texture, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Texture", 1280, 720, true).unwrap();

    let image = imagefmt::read("examples/image.png", imagefmt::ColFmt::RGBA).unwrap();
    let program = Program::basic().unwrap();
    let texture = Texture::new(image.w as u32, image.h as u32, image.buf.as_ref());

    let triangle = Mesh::new(PrimitiveType::Triangles, &[
        Vertex::new(-1.0, -1.0, 0.0).texcoords(0.0, 0.0),
        Vertex::new( 0.0,  1.0, 0.0).texcoords(0.5, 1.0),
        Vertex::new( 1.0, -1.0, 0.0).texcoords(1.0, 0.0),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color(0.0, 0.0, 0.0, 1.0);
        rendust::clear();

        program.bind();
        texture.bind();

        triangle.render();

        window.swap_buffers();
    }
}
