extern crate rendust;

use rendust::math;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};
use rendust::camera::Camera;

fn main() {
    let mut window = Window::new("Rendust example - Scene", 1280, 720, true).unwrap();

    let program = Program::basic().unwrap();

    let projection = math::perspective(90.0, 1280.0 / 720.0, 0.1, 1000.0);
    program.set_uniform_matrix("projection", projection.as_ref());

    let mut camera = Camera::new();
    camera.position.y = 1.0;

    let floor = Mesh::new(PrimitiveType::Quads, &[
        Vertex::new([-25.0, 0.0,  25.0], [0.4, 0.4, 0.4, 1.0]),
        Vertex::new([-25.0, 0.0, -25.0], [0.4, 0.4, 0.4, 1.0]),
        Vertex::new([ 25.0, 0.0, -25.0], [0.4, 0.4, 0.4, 1.0]),
        Vertex::new([ 25.0, 0.0,  25.0], [0.4, 0.4, 0.4, 1.0]),
    ], None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color([0.0, 0.0, 0.0, 1.0]);
        rendust::clear();

        program.bind();
        program.set_uniform_matrix("view", camera.view_matrix().as_ref());

        floor.render();

        window.swap_buffers();
    }
}
