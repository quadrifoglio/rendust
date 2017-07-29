extern crate rendust;

use rendust::math;
use rendust::window::Window;
use rendust::shaders::Program;
use rendust::mesh::{Vertex, PrimitiveType, Mesh};

fn main() {
    let mut window = Window::new("Rendust example - Scene", 1280, 720, true).unwrap();

    let program = Program::basic().unwrap();

    let projection = math::perspective(90.0, 1280.0 / 720.0, 1.0, 1000.0);
    program.set_uniform_matrix("projection", projection.as_ref());

    let mut grid_vertices = Vec::new();
    for x in -50..50 {
        grid_vertices.push(Vertex::new([x as f32, -3.0, -50.0], [0.0, 0.0, 1.0, 1.0]));
        grid_vertices.push(Vertex::new([x as f32, -3.0, 50.0], [0.0, 0.0, 1.0, 1.0]));
    }

    for z in -50..50 {
        grid_vertices.push(Vertex::new([-50.0, -3.0, z as f32], [0.0, 0.0, 1.0, 1.0]));
        grid_vertices.push(Vertex::new([50.0, -3.0, z as f32], [0.0, 0.0, 1.0, 1.0]));
    }

    let grid = Mesh::new(PrimitiveType::Lines, grid_vertices.as_ref(), None);

    while !window.should_exit {
        window.handle_events(|_| ());

        rendust::set_clear_color([0.0, 0.0, 0.0, 1.0]);
        rendust::clear();

        program.bind();
        grid.render();

        window.swap_buffers();
    }
}
