// External libraries
#[macro_use]
extern crate lazy_static;

extern crate gl;
extern crate libc;
extern crate glutin;
extern crate cgmath;

use std::io;

/// Error type used in this library
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    WindowCreation(glutin::CreationError),
    GlContext(glutin::ContextError),
    GlShader(String)
}

/// Result type used in this library
pub type Result<T> = std::result::Result<T, Error>;

// Root scope types and functions

/// Represents a color
/// RGBA, 4 32 bits floating point values
#[repr(C)]
pub struct Color {
    rgba: [f32; 4]
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            rgba: [r, g, b, a]
        }
    }
}

/// Set the clearing color
pub fn set_clear_color(c: Color) {
    unsafe {
        gl::ClearColor(c.rgba[0], c.rgba[1], c.rgba[2], c.rgba[3]);
    }
}

/// Clear the screen, both the color buffer
/// and the depth buffer
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    // Bind a default blank texture, in case no other
    // texture is being use
    mesh::BlankTexture.bind();
}

// Windowing subsystem
pub mod window;

// Shader program management
pub mod shaders;

// Mesh rendering
pub mod mesh;

// Math functionality
pub mod math;
