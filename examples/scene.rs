extern crate rendust;

use rendust::Context;
use rendust::math::{self, Vec3};
use rendust::window::Window;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};
use rendust::camera::Camera;
use rendust::lighting::Ambient;

fn main() {
    let mut window = Window::new("Rendust example - Scene", 1280, 720, true).unwrap();

    let ctx = Context::new().unwrap();
    ctx.set_projection(math::mat4_perspective(90.0, 1280.0 / 720.0, 0.1, 1000.0));
    ctx.set_ambient_light(Ambient::new([0.1, 0.1, 0.1, 1.0], 0.5));

    let camera = Camera::new(Vec3::new(1.0, 1.0, 3.0));

    let floor = Mesh::new(PrimitiveType::Quads, &[
        Vertex::new(-25.0, 0.0,  25.0).color(0.4, 0.4, 0.4, 1.0),
        Vertex::new(-25.0, 0.0, -25.0).color(0.4, 0.4, 0.4, 1.0),
        Vertex::new( 25.0, 0.0, -25.0).color(0.4, 0.4, 0.4, 1.0),
        Vertex::new( 25.0, 0.0,  25.0).color(0.4, 0.4, 0.4, 1.0),
    ], None);

    let cube = Mesh::new(PrimitiveType::Quads, &[
        Vertex::new(-0.5, -0.5, 0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new(-0.5,  0.5, 0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new( 0.5,  0.5, 0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new( 0.5, -0.5, 0.5).color(1.0, 1.0, 1.0, 1.0),

        Vertex::new(-0.5, -0.5, -0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new(-0.5,  0.5, -0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new( 0.5,  0.5, -0.5).color(1.0, 1.0, 1.0, 1.0),
        Vertex::new( 0.5, -0.5, -0.5).color(1.0, 1.0, 1.0, 1.0),
    ], Some(&[
        0, 1, 2, 3,
        4, 5, 6, 7,

        0, 1, 5, 4,
        3, 2, 6, 7,

        0, 4, 7, 3,
        1, 5, 6, 2
    ]));

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color(0.0, 0.0, 0.0, 1.0);
        rendust::clear();

        ctx.set_view(camera.view_matrix());

        floor.render();
        cube.render();

        window.swap_buffers();
    }
}
