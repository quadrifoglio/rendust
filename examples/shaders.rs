extern crate rendust;

use rendust::window::Window;
use rendust::shaders::Program;

fn main() {
    let mut window = Window::new("Rendust example - Shaders", 1280, 720, true).unwrap();

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

    while !window.should_exit {
        window.handle_events(|_| ());
        window.swap_buffers();
    }
}
