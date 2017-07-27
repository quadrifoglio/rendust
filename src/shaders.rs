use gl;
use gl::types::*;
use super::{Result, Error};

pub struct Program(GLuint);

impl Program {
    pub fn new(vert_src: &str, frag_src: &str) -> Result<Program> {
        unsafe {
            // Create both shaders, return an error if it fails
            let (vert, frag) = match (gl::CreateShader(gl::VERTEX_SHADER), gl::CreateShader(gl::FRAGMENT_SHADER)) {
                (0, 0) | (0, _) | (_, 0) => return Err(Error::GlShader("Failed to create shader object".to_owned())),
                (vert, frag) => (vert, frag)
            };

            // Bind the source code of each shader
            gl::ShaderSource(
                vert,
                1 as GLsizei,
                &[vert_src.as_ptr() as *const GLchar] as *const *const GLchar,
                &(vert_src.len() as GLint)
            );

            gl::ShaderSource(
                vert,
                1 as GLsizei,
                &[frag_src.as_ptr() as *const GLchar] as *const *const GLchar,
                &(frag_src.len() as GLint)
            );

            // Compile both shaders
            gl::CompileShader(vert);
            gl::CompileShader(frag);

            // Check for compilation errors in the vertex shader
            let mut compiled: GLint = 0;
            gl::GetShaderiv(vert, gl::COMPILE_STATUS, &mut compiled as *mut GLint);

            if compiled <= 0 {
                return Err(Error::GlShader("Failed to compile vertex shader".to_owned()));
            }

            // Check for compilation errors in the fragment shader
            let mut compiled: GLint = 0;
            gl::GetShaderiv(frag, gl::COMPILE_STATUS, &mut compiled as *mut GLint);

            if compiled <= 0 {
                return Err(Error::GlShader("Failed to compile fragment shader".to_owned()));
            }

            // Link the two shaders into a program
            let program = match gl::CreateProgram() {
                0 => return Err(Error::GlShader("Failed to create shader program".to_owned())),
                p => p
            };

            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);

            // Check link success
            let mut linked: GLint = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut linked as *mut GLint);

            if linked <= 0 {
                return Err(Error::GlShader("Failed to link shader program".to_owned()));
            }

            // Detach the shaders
            gl::DetachShader(program, vert);
            gl::DetachShader(program, frag);

            Ok(Program(program))
        }
    }
}
