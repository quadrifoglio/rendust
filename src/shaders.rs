use std::ffi::{CStr, CString};

use gl;
use gl::types::*;

use super::{Result, Error};

/// Represents an OpenGL shader program
pub struct Program {
    id: GLuint
}

impl Program {
    /// Create a new shader program based on a vertex shader
    /// and a fragment shader
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
                &CStr::from_ptr(vert_src.as_ptr() as *const i8).as_ptr(),
                &(vert_src.len() as GLint)
            );

            gl::ShaderSource(
                frag,
                1 as GLsizei,
                &CStr::from_ptr(frag_src.as_ptr() as *const i8).as_ptr(),
                &(frag_src.len() as GLint)
            );

            // Compile both shaders
            gl::CompileShader(vert);
            gl::CompileShader(frag);

            // Check for compilation errors in the vertex shader
            let mut compiled: GLint = 42;
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

            // Return
            Ok(Program {
                id: program
            })
        }
    }

    /// Set the value of the uniform matrix defined by the specified
    /// name.
    pub fn set_uniform_matrix(&self, name: &str, matrix: &[f32; 16]) {
        unsafe {
            // Get the location of the uniform
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());

            // If the uniform exists
            if loc >= 0 {
                // Set the uniform's value
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr() as *const GLfloat);
            }
        }
    }

    /// Set the value of the uniform float defined by the specified
    /// name
    pub fn set_uniform_float(&self, name: &str, float: f32) {
        unsafe {
            // Get the location of the uniform
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());

            // If the uniform exists
            if loc >= 0 {
                // Set the uniform's value
                gl::Uniform1f(loc, float);
            }
        }
    }

    /// Set the value of the uniform vector defined by the specified
    /// name.
    pub fn set_uniform_vector(&self, name: &str, num: u32, vals: &[f32]) {
        unsafe {
            // Get the location of the uniform
            let loc = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());

            // If the uniform exists
            if loc >= 0 {
                // Set the uniform's value
                match num {
                    2 => gl::Uniform2f(loc, vals[0], vals[1]),
                    3 => gl::Uniform3f(loc, vals[0], vals[1], vals[2]),
                    4 => gl::Uniform4f(loc, vals[0], vals[1], vals[2], vals[3]),
                    _ => ()
                }
            }
        }
    }

    /// Bind the shader program in order to use it
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
