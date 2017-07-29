use super::Result;
use shaders::Program;
use math::{self, Mat4};
use lighting::Ambient;

/// Contains the main state
/// of the rendering library
pub struct Context {
    shader_program: Program
}

impl Context {
    /// Create a new context, initiate the library
    pub fn new() -> Result<Context> {
        let vert = r#"
            #version 140

            uniform mat4 projection;
            uniform mat4 view;
            uniform mat4 model;

            in vec3 position;
            in vec4 color;
            in vec2 texcoords;

            out vec4 frag_color;
            out vec2 frag_texcoords;

            void main() {
                gl_Position = projection * view * model * vec4(position, 1.0);

                frag_color = color;
                frag_texcoords = texcoords;
            }
        "#;

        let frag = r#"
            #version 140

            uniform sampler2D tex;

            uniform vec4 ambient_light_color;
            uniform float ambient_light_strength;

            in vec4 frag_color;
            in vec2 frag_texcoords;

            out vec4 out_color;

            void main() {
                vec4 obj_color = texture2D(tex, frag_texcoords) * frag_color;

                if(ambient_light_strength > 0) {
                    out_color = ambient_light_strength * ambient_light_color * obj_color;
                }
                else {
                    out_color = obj_color;
                }
            }
        "#;

        // Create the shader program
        let program = try!(Program::new(vert, frag));
        program.bind();

        // Set the initial values for the uniform matrices
        let matrix = math::mat4_identity();
        program.set_uniform_matrix("projection", matrix.as_ref());
        program.set_uniform_matrix("view", matrix.as_ref());
        program.set_uniform_matrix("model", matrix.as_ref());

        // Bind the program
        program.bind();

        Ok(Context {
            shader_program: program
        })
    }

    /// Set the projection matrix
    pub fn set_projection(&self, m: Mat4) {
        self.shader_program.set_uniform_matrix("projection", m.as_ref());
    }

    /// Set the view matrix
    pub fn set_view(&self, m: Mat4) {
        self.shader_program.set_uniform_matrix("view", m.as_ref());
    }

    /// Set the model matrix
    pub fn set_model(&self, m: Mat4) {
        self.shader_program.set_uniform_matrix("model", m.as_ref());
    }

    /// Use an ambient light
    pub fn set_ambient_light(&self, l: Ambient) {
        self.shader_program.set_uniform_vector("ambient_light_color", 4, &l.color);
        self.shader_program.set_uniform_float("ambient_light_strength", l.strength);
    }
}
